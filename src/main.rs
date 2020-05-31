use eu4::{GameData, Gamestate};
fn compute_trade(_data: &GameData, gamestate: &Gamestate) {
    for trade_node in &gamestate.trade.node {
        println!("Trade node: {:?}", trade_node.definitions);
        println!("Total value in node: {}", trade_node.current);
    }
}

fn main() -> Result<(), paradox::ParseError> {
    let mut _eu4data = eu4::GameData::new(
        &paradox::get_default_steam_dir().join("Europa Universalis IV"))?;
    //let gamestate = paradox::load_savegame::<eu4::Gamestate>(
    //    &paradox::get_default_save_dir().join("Europa Universalis IV/save games/SunGod.eu4"),
//  //      &paradox::get_default_save_dir().join("Europa Universalis IV/save games/older_autosave.eu4"),
    //    &mut eu4data.base_info
    //    )?;
    //compute_trade(&eu4data, &gamestate);
    Ok(())
}
