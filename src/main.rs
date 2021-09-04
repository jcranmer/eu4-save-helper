use eu4::{Eu4Atom, GameData, Gamestate};
mod trade;

/*fn get_tier(gamedata: &GameData, gamestate: &Gamestate) {
    // Modifiers:
    let country_id = gamedata.tags.get_index("BRB".into());
}*/

fn main() -> Result<(), paradox::ParseError> {
    let mut eu4data = eu4::GameData::new(
        &paradox::get_default_steam_dir().join("Europa Universalis IV"))?;
    let gamestate = paradox::load_savegame::<eu4::Eu4Trait, eu4::Gamestate>(
        &std::path::Path::new("/tmp/TrailOfTears.eu4-37"),
//        &paradox::get_default_save_dir().join("Europa Universalis IV/save games/Mamluks.eu4"),
        &mut eu4data.base_info
        )?;
    trade::optimize_trade(&eu4data, &gamestate,
                          gamestate.player.as_ref());
    // Debugging: trade power for everybody
    //for (tag, country) in &gamestate.countries {
    //    if country.num_of_cities == 0 { continue; }
    //    let modifiers = country.get_modifiers(&eu4data, &gamestate, &tag);
    //    println!("{} global_prov_trade_power_modifier = {:?}", tag,
    //             modifiers[&eu4::eu4_atom!("global_prov_trade_power_modifier")]);
    //}
    //let country = &gamestate.countries[&gamestate.player];
    //let modifiers = country.get_modifiers(&eu4data, &gamestate, &gamestate.player);
    //for (key, value) in modifiers.modifiers {
    //    println!("Modifier {} = {:?}", key, value);
    //}

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
