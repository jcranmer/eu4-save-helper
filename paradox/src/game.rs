use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use crate::{GameTrait, ParadoxParse, ParseError, Parser, ParserAtom};

type Result<T> = std::result::Result<T, ParseError>;

/// Core game mechanics for Paradox games.
///
/// This struct shouldn't be used by most people, as the core elements will be
/// exposed by crates deriving GameData (which uses this struct internally).
pub struct GameData {
    game_directory: PathBuf,
}

impl GameData {
    /// Initialize this struct from the given directory.
    pub fn load(game_dir: &Path) -> Result<Self> {
        if !game_dir.is_dir() {
            let err = Error::new(ErrorKind::InvalidInput,
                format!("Path {} is not a directory", game_dir.display()));
            return Err(err.into());
        }

        Ok(GameData {
            game_directory: game_dir.to_path_buf()
        })
    }

    /// Parse a directory (usually in $GAME/common/*.txt) into a parsable type.
    pub fn parse_directory<G: GameTrait>(
        &mut self, path: &str,
        target: &mut dyn ParadoxParse<G>) -> Result<&mut Self>
    {
        crate::load_directory(&self.game_directory.join(path), target, self)?;
        Ok(self)
    }
}

pub trait BoxedValue: Default {
    type Trait : GameTrait;
    const TYPE_VALUE: u32;
    const DEFAULT_STRING: &'static str = "";
}

#[derive(Default, Debug)]
pub struct TypeDefinition<T: BoxedValue + ParadoxParse<T::Trait>> {
    map: HashMap<ParserAtom, usize>,
    values: Vec<(ParserAtom, T)>
}

impl <T: BoxedValue + ParadoxParse<T::Trait>> TypeDefinition<T> {
    pub fn get_names(&self) -> impl Iterator<Item = &ParserAtom> {
        self.values.iter()
            .map(|(name, _)| name)
    }

    pub fn get_index(&self, name: ParserAtom) -> usize {
        *self.map.get(&name).unwrap()
    }
}

impl <'a, T> std::ops::Index<&'a ParserAtom> for TypeDefinition<T>
    where T: BoxedValue + ParadoxParse<T::Trait>
{
    type Output = T;
    fn index(&self, idx: &'a ParserAtom) -> &T {
        &self.values[self.map[idx]].1
    }
}

impl <T: BoxedValue + ParadoxParse<T::Trait>> ParadoxParse<T::Trait> for TypeDefinition<T> {
    fn read(&mut self, parser: &mut Parser<T::Trait>) -> Result<()> {
        parser.parse_key_scope(|key, parser| {
            let index = self.values.len();
            if self.map.insert(key.clone(), index).is_some() {
                return Err(ParseError::Constraint(
                        format!("Duplicate key {} in map", key)));
            }
            let mut val = T::default();
            val.read(parser)?;
            self.values.push((key, val));
            Ok(())
        })
    }
}
