use std::convert::TryInto;
use std::io::{Error, ErrorKind};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use crate::{IdBox, ParadoxParse, ParseError, Parser, Token};

type Result<T> = std::result::Result<T, ParseError>;

/// Core game mechanics for Paradox games.
///
/// This struct shouldn't be used by most people, as the core elements will be
/// exposed by crates deriving GameData (which uses this struct internally).
pub struct GameData {
    game_directory: PathBuf,
    id_boxes: Vec<IdBox>
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
            game_directory: game_dir.to_path_buf(),
            id_boxes: Vec::new()
        })
    }

    /// Parse a directory (usually in $GAME/common/*.txt) into a parsable type.
    pub fn parse_directory(&mut self, path: &str,
                           target: &mut dyn ParadoxParse) -> Result<&mut Self> {
        crate::load_directory(&self.game_directory.join(path), target, self)?;
        Ok(self)
    }

    pub fn get_id_box_mut<T: BoxedValue>(&mut self) -> &mut IdBox {
        let index = T::TYPE_VALUE.try_into().unwrap();
        while self.id_boxes.len() <= index {
            self.id_boxes.push(IdBox::new());
        }
        &mut self.id_boxes[index]
    }
}

pub trait BoxedValue {
    const TYPE_VALUE: u32;
    const DEFAULT_STRING: &'static str = "";
}

#[derive(Ord, PartialOrd, Copy, Clone)]
pub struct IdKey<T: BoxedValue> {
    index: u16,
    _data: PhantomData<T>
}

impl <T: BoxedValue> std::fmt::Debug for IdKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let class_name = std::any::type_name::<T>();
        let simple = class_name.split("::").last().unwrap();
        write!(f, "{}Key({})", simple, self.index)
    }
}

impl <T: BoxedValue> std::hash::Hash for IdKey<T> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.index.hash(hasher);
    }
}

impl <T: BoxedValue> PartialEq for IdKey<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.index == rhs.index
    }
}

impl <T: BoxedValue> Eq for IdKey<T> { }

impl <T: BoxedValue> IdKey<T> {
    pub fn new(id_box: &mut IdBox, string: &str) -> Self {
        Self {
            index: id_box.add_string(string),
            _data: PhantomData
        }
    }

    pub fn new_via_gamedata(game_data: &mut GameData, string: &str) -> Self {
        Self::new(game_data.get_id_box_mut::<T>(), string)
    }
}

#[derive(Ord, PartialOrd, Copy, Clone, Default)]
pub struct IdRef<T: BoxedValue> {
    index: u16,
    _data: PhantomData<T>
}

impl <T: BoxedValue> std::fmt::Debug for IdRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let class_name = std::any::type_name::<T>();
        let simple = class_name.split("::").last().unwrap();
        write!(f, "{}Ref({})", simple, self.index)
    }
}

impl <T: BoxedValue> std::hash::Hash for IdRef<T> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.index.hash(hasher);
    }
}

impl <T: BoxedValue> PartialEq for IdRef<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.index == rhs.index
    }
}

impl <T: BoxedValue> Eq for IdRef<T> { }

impl <T: BoxedValue> ParadoxParse for IdRef<T> {
    fn read_from(&mut self, parser: &mut Parser, val: Token) -> Result<()> {
        let key = val.try_to_string()?;
        if key == T::DEFAULT_STRING {
            self.index = 0;
            return Ok(());
        }

        let id_box = parser.get_game_data().get_id_box_mut::<T>();
        self.index = id_box.get_index(key)
            .ok_or_else(|| parser.validation_error(
                    std::any::type_name::<Self>(),
                    &key,
                    "not known to be in gamedata",
                    true, None).unwrap_err())?;
        Ok(())
    }
}
