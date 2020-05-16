use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use crate::{ParadoxParse, ParseError};

type Result<T> = std::result::Result<T, ParseError>;

/// Core game mechanics for Paradox games.
///
/// This struct shouldn't be used by most people, as the core elements will be
/// exposed by crates deriving GameData (which uses this struct internally).
pub struct GameData {
    game_directory: PathBuf
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
    pub fn parse_directory(&self, path: &'static str,
                           target: &mut dyn ParadoxParse) -> Result<&Self> {
        crate::load_directory(&self.game_directory.join(path), target)?;
        Ok(self)
    }
}
