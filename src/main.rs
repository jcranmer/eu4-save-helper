#[macro_use]
extern crate auto_paradox;

use std::fs::File;
use std::path::Path;

use paradox::Date;
use paradox::FixedPoint;

mod events;
mod game;
mod gamestate;
mod lexer;

fn main() -> Result<(), paradox::ParseError> {
    let mut eu4data = game::GameData::load(
        &paradox::get_default_steam_dir().join("Europa Universalis IV"));
    eu4data.validate_gamefiles()?;
    //let data = eu4data.events()?;
    //let file = File::open(&paradox::get_default_save_dir().join("Europa Universalis IV/save games/older_autosave.eu4"))?;
    //gamestate::load_savegame(file)?;
    let gamestate = paradox::load_savegame::<eu4::Gamestate>(
        &paradox::get_default_save_dir().join("Europa Universalis IV/save games/SunGod.eu4")
//        &paradox::get_default_save_dir().join("Europa Universalis IV/save games/older_autosave.eu4")
        )?;
    Ok(())
}
