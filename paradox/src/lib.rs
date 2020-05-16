//! This crate provides an implementation of core concepts within Paradox's
//! Clausewitz Engine (used in Europa Universalis, Crusader Kings, and Stellaris
//! among other games).
//!
//! The utilities provided are:
//! * A representation of the date system used in the games.
//! * An implementation of the fixed-point arithmetic they use.
//! * A parser for their internal format.

// Set up #[derive(ParadoxParse)] support.
#[allow(unused_imports)]
#[macro_use]
extern crate paradox_derive;
pub use paradox_derive::*;

mod date;
mod fixed;
mod parser;
mod parser_impl;
mod save;

pub use date::*;
pub use fixed::*;
pub use parser::*;
pub use save::*;

use std::path::PathBuf;

#[cfg(unix)]
pub fn get_default_steam_dir() -> PathBuf {
    // Linux default directories are a mess. The Debian/Ubuntu .debs appear to
    // install in .steam by default.
    let mut steam_dir = dirs::home_dir().unwrap().join(".steam");
    if !steam_dir.is_dir() {
        // Try in $XDG_DATA_HOME/steam instead.
        steam_dir = dirs::data_local_dir().unwrap().join("Steam");
        if !steam_dir.is_dir() {
            panic!("Can't find directory of steam");
        }
    }

    steam_dir.push("steam");
    steam_dir.push("SteamApps");
    steam_dir.push("common");
    steam_dir
}

/// Get the root directory of save games for Paradox games.
///
/// You need to append the game name and 'save games' to the location to get the
/// correct location in the end.
#[cfg(unix)]
pub fn get_default_save_dir() -> PathBuf {
    dirs::data_local_dir().unwrap().join("Paradox Interactive")
}
