use paradox::{Date, FixedPoint, ParadoxParse};
use std::collections::HashMap;

#[derive(ParadoxParse, Default)]
pub struct Gamestate {
    // XXX: 0x2f3f slots in here. Unknown member.
    #[id(0x2c9b)] pub gameplaysettings: (),
    #[id(0x006e)] pub speed: i32,
    #[id(0x2dc6)] pub multiplayer_random_seed: u32,
    #[id(0x2dc7)] pub multiplayer_random_count: i32,
    #[id(0x3564)] pub current_age: String, // XXX
    #[id(0x362c)] pub next_age_progress: FixedPoint,
    #[id(0x36d8)] pub id_counters: Vec<u32>,
    #[id(0x28a6)] pub unit: i32,
    #[id(0x30f6)] pub unit_template_id: i32,
    #[id(0x29cc)] pub flags: HashMap<String, Date>,
    #[id(0x28e4)] pub start_date: (), // XXX: Date
    #[id(0x36f1)] pub map_area_data: HashMap<String, ()>,
    #[id(0x3426)] pub total_military_power: f64,
    #[id(0x3427)] pub average_military_power: f64,
    #[id(0x3472)] pub institution_origin: Vec<i32>, // XXX: ProvinceRef
    #[id(0x3447)] pub institutions: Vec<i32>,
    #[id(0x347f)] pub institutions_penalties: Vec<FixedPoint>,
    #[id(0x2871)] pub trade: Trade,
    #[id(0x2ecf)] pub production_leader_tag: Vec<String>, // XXX: CountryRef
    #[id(0x2e6c)] pub tradegoods_total_produced: Vec<FixedPoint>,
    #[id(0x3096)] pub change_price: HashMap<String, ()>,
    // id
    #[id(0x2bb4)] pub dynasty: (),
    #[id(0x2cc7)] #[repeated] pub rebel_faction: Vec<()>,
    #[id(0x343a)] pub great_powers: (),
    #[id(0x2d32)] pub empire: (),
    #[id(0x35b7)] pub celestial_empire: (),
    #[id(0x308d)] pub hre_leagues_status: i32,
    #[id(0x308e)] pub hre_religion_status: i32,
    #[id(0x331c)] #[repeated] pub trade_league: Vec<()>,
    #[id(0x2e4d)] pub religions: HashMap<String, ()>,
    #[id(0x3729)] pub religion_instance_data: HashMap<String, ()>,
    #[id(0x2afe)] pub fired_events: (),
    #[id(0x2e63)] pub pending_events: (),
    #[id(0x2833)] pub provinces: HashMap<String, ()>,
    #[id(0x2e4e)] pub countries: HashMap<String, ()>,
    #[id(0x28da)] pub active_advisors: HashMap<String, ()>,
    #[id(0x28ec)] pub diplomacy: (),
    #[id(0x291a)] pub combat: (),
    #[id(0x28f7)] #[repeated] pub active_war: Vec<()>,
    #[id(0x28f8)] #[repeated] pub previous_war: Vec<()>,
    #[id(0x0137)] pub income_statistics: Statistics,
    #[id(0x0138)] pub nation_size_statistics: Statistics,
    #[id(0x2b2a)] pub score_statistics: Statistics,
    #[id(0x0139)] pub inflation_statistics: Statistics,
    #[id(0x327c)] pub expanded_dip_action_groups: Vec<i32>,
    #[id(0x3328)] pub achievement_ok: bool,
    //2a8d, 3741, 374e
    #[id(0x37e3)] pub tech_level_dates: (), // it's a [(String, Date); 3]
    #[id(0x37f7)] pub idea_dates: HashMap<String, Date>,
    #[id(0x0179)] pub checksum: String,
}

#[derive(ParadoxParse, Default)]
pub struct Trade {
    #[id(0x015e)] #[repeated] pub node: Vec<TradeNode>,
}

#[derive(ParadoxParse, Default)]
pub struct TradeNode {
    #[id(0x2835)] pub definitions: String,
    #[id(0x2cad)] #[optional] pub current: FixedPoint,
    #[id(0x2da5)] #[optional] pub local_value: FixedPoint,
    #[id(0x2aa8)] #[optional] pub outgoing: FixedPoint,
    #[id(0x2b20)] #[optional] pub value_added_outgoing: FixedPoint,
    #[id(0x2da6)] pub retention: FixedPoint,
    #[id(0x2da7)] #[repeated] pub steer_power: Vec<FixedPoint>,
    #[id(0x37b0)] pub num_collectors: i32,
    #[id(0x2a15)] #[optional] pub total: FixedPoint,
    #[id(0x30e6)] #[optional] pub p_pow: FixedPoint,
    #[id(0x02a4)] #[optional] pub max: FixedPoint,
    #[id(0x2afb)] #[optional] pub collector_power: FixedPoint,
    #[id(0x2da8)] #[optional] pub pull_power: FixedPoint,
    #[id(0x2da9)] #[optional] pub retain_power: FixedPoint,
    #[id(0x2daa)] #[optional] pub highest_power: FixedPoint,
    #[id(0x372d)] #[optional] pub _wtf_is_this_broken_thing: FixedPoint,
    #[id(0x2aa7)] #[repeated] pub incoming: Vec<()>,
    #[id(0x2b06)] pub trade_goods_size: Vec<FixedPoint>,
    #[id(0x2b3c)] #[optional] pub top_provinces: Vec<String>,
    #[id(0x2b3d)] #[optional] pub top_provinces_values: Vec<FixedPoint>,
    #[id(0x2b3e)] #[optional] pub top_power: Vec<String>,
    #[id(0x2b3f)] #[optional] pub top_power_values: Vec<String>,
    #[id(0x2f92)] #[optional] pub trade_company_region: bool,
    #[id(0x3925)] pub most_recent_treasure_ship_passage: (), // XXX: Date
    #[collect] pub country_info: HashMap<String, CountryTradeNode>,
}

#[derive(ParadoxParse, Default)]
pub struct CountryTradeNode {
    #[id(0x00e1)] #[optional] pub r#type: i32,
    #[id(0x30e2)] #[optional] pub val: FixedPoint,
    #[id(0x2cb8)] #[optional] pub potential: FixedPoint,
    #[id(0x2d99)] #[optional] pub prev: FixedPoint,
    #[id(0x30e1)] #[optional] pub max_pow: FixedPoint,
    #[id(0x2846)] #[optional] pub max_demand: FixedPoint,
    #[id(0x2dac)] #[optional] pub province_power: FixedPoint,
    #[id(0x2b0b)] #[optional] pub ship_power: FixedPoint,
    #[id(0x2dad)] #[optional] pub power_fraction: FixedPoint,
    #[id(0x2daf)] #[optional] pub money: FixedPoint,
    #[id(0x2a15)] #[optional] pub total: FixedPoint,
    #[id(0x2da7)] #[optional] pub steer_power: i32,
    #[id(0x30e5)] #[optional] pub add: FixedPoint,
    #[id(0x2f70)] #[optional] pub already_sent: FixedPoint,
    #[id(0x3731)] #[optional] pub _something_something: FixedPoint,
    #[id(0x3732)] #[optional] pub _something_something2: FixedPoint,
    #[id(0x2c09)] #[optional] pub has_trader: bool,
    #[id(0x2c94)] #[optional] pub has_capital: bool,
    #[id(0x2785)] #[optional] pub light_ship: i32,
    #[id(0x30df)] #[optional] pub t_in: FixedPoint,
    #[id(0x30e4)] #[optional] pub t_from: HashMap<String, FixedPoint>,
    #[id(0x30e0)] #[optional] pub t_out: FixedPoint,
    #[id(0x30e3)] #[optional] pub t_to: HashMap<String, FixedPoint>,
    #[id(0x3747)] #[optional] pub _trading_strategy: String,
    #[id(0x3748)] #[optional] pub _trading_strategy_date: (), // XXX: Date
    #[id(0x2968)] #[optional] pub modifier: (),
}

#[derive(ParadoxParse, Default)]
pub struct Statistics {
    #[id(0x0134)] #[repeated] pub ledger_data: Vec<LedgerData>,
}

#[derive(ParadoxParse, Default)]
pub struct LedgerData {
    pub name: String, // XXX: CountryRef
    #[id(0x00f0)] #[optional] pub data: HashMap<String, i32>,
}
