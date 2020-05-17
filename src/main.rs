fn main() -> Result<(), paradox::ParseError> {
    let _eu4data = eu4::GameData::new(
        &paradox::get_default_steam_dir().join("Europa Universalis IV"))?;
    let _gamestate = paradox::load_savegame::<eu4::Gamestate>(
        &paradox::get_default_save_dir().join("Europa Universalis IV/save games/SunGod.eu4")
//        &paradox::get_default_save_dir().join("Europa Universalis IV/save games/older_autosave.eu4")
        )?;
    Ok(())
}
