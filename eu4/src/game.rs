use std::collections::HashMap;

#[derive(paradox::GameData)]
pub struct GameData {
    base_info: paradox::GameData,

    #[parse = "common/religions"]
    religions: crate::ReligionList,

    #[parse = "common/tradenodes"]
    trade: crate::TradeNodeList,

    #[parse = "map/area.txt"]
    areas: HashMap<String, ()>,
}

