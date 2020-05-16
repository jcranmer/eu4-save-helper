use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use crate::{
    lexer::{Lexer, ParadoxScope, parse_file}
};

use paradox::{ParadoxParse, ParseError, TextLexer};

pub struct GameData {
    game_path: PathBuf,
    loaded_data: HashMap<TypeId, Box<dyn Any + 'static>>
}

fn read_game_file(path: &Path, data: &mut dyn ParadoxScope) -> std::io::Result<()> {
    let file = File::open(path)?; 
    let lexer = Lexer::new(file);
    parse_file(lexer, data)?;
    Ok(())
}

fn read_directories_old(game_path: &Path, path: &Path,
                        data: &mut dyn ParadoxScope) -> std::io::Result<()> {
    let file_dir = game_path.join(path);
    for entry in file_dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if !entry.metadata()?.is_file() {
            eprintln!("Unexpected non-file in directory: {}",
                      path.display());
            continue;
        } else if path.extension().is_none() {
            eprintln!("Unexpected non-txt file in directory: {}",
                      path.display());
            continue;
        } else if path.extension().unwrap() != "txt" {
            eprintln!("Unexpected non-txt file in directory: {}",
                      path.display());
            continue;
        }

        read_game_file(&path, data)?;
    }
    Ok(())
}

fn read_directories<P: ParadoxParse>(game_path: &Path, path: &Path,
                    data: &mut P) -> Result<(), ParseError> {
    let file_dir = game_path.join(path);
    for entry in file_dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if !entry.metadata()?.is_file() {
            eprintln!("Unexpected non-file in directory: {}",
                      path.display());
            continue;
        } else if path.extension().is_none() {
            eprintln!("Unexpected non-txt file in directory: {}",
                      path.display());
            continue;
        } else if path.extension().unwrap() != "txt" {
            eprintln!("Unexpected non-txt file in directory: {}",
                      path.display());
            continue;
        }

        let filename = path.to_string_lossy().into();
        let file = File::open(path)?; 
        let lexer = TextLexer::new(file, filename);
        paradox::Parser::new(Box::new(lexer)).parse(data)
            .unwrap();
    }
    Ok(())
}

macro_rules! cached_fn {
    (old $fn_name:ident : $t:ty = $e:expr) => {
        pub fn $fn_name(&mut self) -> std::io::Result<&mut $t> {
            self.get_old($e)
        }
    };
    ($fn_name:ident : $t:ty = $e:expr) => {
        pub fn $fn_name(&mut self) -> Result<&mut $t, ParseError> {
            self.get($e)
        }
    };
}
impl GameData {
    pub fn load(game_dir: &Path) -> GameData {
        GameData {
            game_path: game_dir.to_path_buf(),
            loaded_data: Default::default()
        }
    }

    fn get_old<T: ParadoxScope + Default + 'static, P: AsRef<Path>>(
            &mut self, path: P) -> std::io::Result<&mut T> {
        let key = TypeId::of::<T>();
        let entry = self.loaded_data.entry(key);
        use std::collections::hash_map::Entry;
        let boxed_value = match entry {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let mut val: Box<T> = Box::new(Default::default());
                read_directories_old(&self.game_path, path.as_ref(), val.as_mut())?;
                entry.insert(val)
            }
        };
        let inner = boxed_value.as_mut().downcast_mut::<T>()
            .expect("Should be the same type as we put in it!");
        Ok(inner)
    }

    fn get<T: ParadoxParse + Default + 'static, P: AsRef<Path>>(
            &mut self, path: P) -> Result<&mut T, ParseError> {
        let key = TypeId::of::<T>();
        let entry = self.loaded_data.entry(key);
        use std::collections::hash_map::Entry;
        let boxed_value = match entry {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let mut val: Box<T> = Box::new(Default::default());
                read_directories(&self.game_path, path.as_ref(), val.as_mut())?;
                entry.insert(val)
            }
        };
        let inner = boxed_value.as_mut().downcast_mut::<T>()
            .expect("Should be the same type as we put in it!");
        Ok(inner)
    }

    cached_fn!(religions: eu4::ReligionList = "common/religions");
    cached_fn!(old events: crate::events::EventList = "events");
    cached_fn!(trade: eu4::TradeNodeList = "common/tradenodes");

    pub fn validate_gamefiles(&mut self) -> Result<(), ParseError> {
        self.religions()?;
        //self.events()?;
        self.trade()?;
        Ok(())
    }
}
