#[macro_use]
extern crate auto_paradox;

use std::fs::File;
use std::path::Path;

use paradox::Date;
mod fixed; use fixed::FixedPoint;

mod events;
mod game;
mod gamestate;
mod lexer;

fn main() -> Result<(), std::io::Error> {
    let mut eu4data = game::GameData::load(
        &Path::new("/home/jcranmer/.steam/steam/SteamApps/common/Europa Universalis IV"));
    eu4data.trade()?;
    eu4data.religions()?;
    //let data = eu4data.events()?;
    //let file = File::open("/home/jcranmer/.local/share/Paradox Interactive/Europa Universalis IV/save games/older_autosave.eu4")?;
    //gamestate::load_savegame(file)?;
    Ok(())
}
