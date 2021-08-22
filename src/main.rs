use eu4::{GameData, Gamestate};
mod trade;

fn main() -> Result<(), paradox::ParseError> {
    let mut eu4data = eu4::GameData::new(
        &paradox::get_default_steam_dir().join("Europa Universalis IV"))?;
    let gamestate = paradox::load_savegame::<eu4::Gamestate>(
        &std::path::Path::new("/tmp/TrailOfTears.eu4-37"),
//        &paradox::get_default_save_dir().join("Europa Universalis IV/save games/Mamluks.eu4"),
//      &paradox::get_default_save_dir().join("Europa Universalis IV/save games/older_autosave.eu4"),
        &mut eu4data.base_info
        )?;
    trade::optimize_trade(&eu4data, &gamestate,
                          gamestate.player.as_ref());

    //let country_ref = eu4data.base_info.get_id_box::<eu4::Country>()
    //    .get_index("MCH").unwrap();
    //let unconquered : paradox::FixedPoint = gamestate.countries.iter()
    //    .filter_map(|(tag, country)| {
    //        // Direct ownership
    //        if tag.index == country_ref { return None; }
    //        // Non-tributary subject (vassal, PU, colonial nation)
    //        if country.overlord.index == country_ref &&
    //            country.tribute_type != 0 {
    //                return None;
    //        }
    //        Some(country.raw_development)
    //    }).fold(Default::default(), |sum, dev| sum + dev);
    //println!("Total unconquered: {}", unconquered);
    Ok(())
}
