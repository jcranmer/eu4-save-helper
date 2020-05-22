use crate::*;
use paradox::{Date, FixedPoint, IdRef, ParadoxParse};
use std::collections::HashMap;

type CountryRef = IdRef<crate::Country>;

#[derive(ParadoxParse, Default)]
pub struct Gamestate {
    #[optional] pub players_countries: (),
    pub gameplaysettings: (),
    pub speed: i32,
    pub multiplayer_random_seed: u32,
    pub multiplayer_random_count: i32,
    pub current_age: String, // XXX
    pub next_age_progress: FixedPoint,
    pub id_counters: Vec<u32>,
    pub unit: i32,
    pub unit_template_id: i32,
    pub flags: HashMap<String, Date>,
    pub start_date: Date,
    pub map_area_data: HashMap<String, ()>,
    pub total_military_power: f64,
    pub average_military_power: f64,
    pub institution_origin: Vec<i32>, // XXX: ProvinceRef
    pub institutions: Vec<i32>,
    pub institutions_penalties: Vec<FixedPoint>,
    pub trade: Trade,
    pub production_leader_tag: Vec<String>, // XXX: CountryRef
    pub tradegoods_total_produced: Vec<FixedPoint>,
    pub change_price: HashMap<String, ()>,
    pub id: (),
    pub dynasty: (),
    #[repeated] pub rebel_faction: Vec<()>,
    pub great_powers: (),
    pub empire: (),
    pub celestial_empire: (),
    pub hre_leagues_status: i32,
    pub hre_religion_status: i32,
    #[repeated] pub trade_league: Vec<()>,
    pub religions: HashMap<IdRef<Religion>, ()>,
    pub religion_instance_data: HashMap<String, ()>,
    pub fired_events: (),
    pub pending_events: (),
    pub provinces: HashMap<String, Province>,
    pub countries: HashMap<String, Country>,
    pub active_advisors: HashMap<String, ()>,
    pub diplomacy: (),
    pub combat: (),
    #[repeated] pub active_war: Vec<()>,
    #[repeated] pub previous_war: Vec<()>,
    pub income_statistics: Statistics,
    pub nation_size_statistics: Statistics,
    pub score_statistics: Statistics,
    pub inflation_statistics: Statistics,
    pub expanded_dip_action_groups: Vec<i32>,
    pub achievement_ok: bool,
    #[optional] pub achievement: (),
    pub unit_manager: (),
    pub trade_company_manager: (),
    pub tech_level_dates: (), // it's a [(String, Date); 3]
    pub idea_dates: HashMap<String, Date>,
    pub checksum: String,
}

#[derive(ParadoxParse, Default)]
pub struct Trade {
    #[repeated] pub node: Vec<TradeNode>,
}

#[derive(ParadoxParse, Default)]
pub struct TradeNode {
    pub definitions: String,
    #[optional] pub current: FixedPoint,
    #[optional] pub local_value: FixedPoint,
    #[optional] pub outgoing: FixedPoint,
    #[optional] pub value_added_outgoing: FixedPoint,
    pub retention: FixedPoint,
    #[repeated] pub steer_power: Vec<FixedPoint>,
    pub num_collectors: i32,
    #[optional] pub total: FixedPoint,
    #[optional] pub p_pow: FixedPoint,
    #[optional] pub max: FixedPoint,
    #[optional] pub collector_power: FixedPoint,
    #[optional] pub pull_power: FixedPoint,
    #[optional] pub retain_power: FixedPoint,
    #[optional] pub highest_power: FixedPoint,
    #[optional] pub pirate_hunt: FixedPoint,
    #[optional] pub total_privateer_power: FixedPoint,
    #[repeated] pub incoming: Vec<()>,
    pub trade_goods_size: Vec<FixedPoint>,
    #[optional] pub top_provinces: Vec<CountryRef>,
    #[optional] pub top_provinces_values: Vec<FixedPoint>,
    #[optional] pub top_power: Vec<CountryRef>,
    #[optional] pub top_power_values: Vec<FixedPoint>,
    #[optional] pub trade_company_region: bool,
    pub most_recent_treasure_ship_passage: Date,
    #[collect] pub country_info: HashMap<String, CountryTradeNode>,
}

#[derive(ParadoxParse, Default)]
pub struct CountryTradeNode {
    #[optional] pub r#type: i32,
    #[optional] pub val: FixedPoint,
    #[optional] pub potential: FixedPoint,
    #[optional] pub prev: FixedPoint,
    #[optional] pub max_pow: FixedPoint,
    #[optional] pub max_demand: FixedPoint,
    #[optional] pub province_power: FixedPoint,
    #[optional] pub ship_power: FixedPoint,
    #[optional] pub power_fraction: FixedPoint,
    #[optional] pub money: FixedPoint,
    #[optional] pub total: FixedPoint,
    #[optional] pub steer_power: i32,
    #[optional] pub add: FixedPoint,
    #[optional] pub already_sent: FixedPoint,
    #[optional] pub _something_something: FixedPoint,
    #[optional] pub _something_something2: FixedPoint,
    #[optional] pub has_trader: bool,
    #[optional] pub has_capital: bool,
    #[optional] pub light_ship: i32,
    #[optional] pub t_in: FixedPoint,
    #[optional] pub t_from: HashMap<CountryRef, FixedPoint>,
    #[optional] pub t_out: FixedPoint,
    #[optional] pub t_to: HashMap<CountryRef, FixedPoint>,
    #[optional] pub trading_policy: String,
    #[optional] pub trading_policy_date: Date,
    #[optional] pub modifier: (),
    #[optional] pub privateer_mission: FixedPoint,
    #[optional] pub privateer_money: FixedPoint,
}

#[derive(ParadoxParse, Default)]
pub struct Country {
    #[optional] pub human: bool,
    #[optional] pub was_player: bool,
    #[optional] pub has_set_government_name: bool,
    pub government_rank: i32,
    #[optional] pub government_name: String,
    pub subject_focus: i32,
    pub trade_mission: FixedPoint,
    pub blockade_mission: FixedPoint,
    pub continent: Vec<i32>,
    #[optional] pub national_focus: String,
    pub institutions: Vec<i32>,
    #[optional] pub technology_cost: FixedPoint,
    #[optional] pub num_of_age_objectives: i32,
    #[repeated] pub active_age_ability: Vec<String>,
    #[optional] pub last_focus_move: Date,
    #[optional] pub last_sent_alliance_offer: Date,
    #[optional] pub history: (),
    #[optional] pub flags: HashMap<String, Date>,
    #[optional] pub hidden_flags: HashMap<String, Date>,
    #[optional] pub variables: HashMap<String, FixedPoint>,
    pub capital: i32, // XXX: ProvinceRef
    #[optional] pub fixed_capital: i32, // XXX: ProvinceRef
    #[optional] pub original_capital: i32, // XXX: ProvinceRef
    pub trade_port: i32, // XXX: ProvinceRef
    #[optional] pub base_tax: FixedPoint,
    #[optional] pub development: FixedPoint,
    #[optional] pub raw_development: FixedPoint,
    pub capped_development: FixedPoint,
    pub realm_development: FixedPoint,
    #[optional] pub in_debt: bool,
    #[optional] pub karma: FixedPoint,
    pub isolationism: i32,
    #[optional] pub potential_incidents: Vec<String>,
    #[optional] pub active_incidents: Vec<String>,
    #[optional] pub past_incidents: Vec<String>,
    #[optional] pub incident_variables: HashMap<String, FixedPoint>,
    #[optional] pub harmony: FixedPoint,
    #[optional] pub harmonized_religions: Vec<i32>,
    pub initialized_rivals: bool,
    pub recalculate_strategy: bool,
    pub colors: (),
    #[optional] pub name: String,
    #[optional] pub adjective: String,
    pub dirty_colony: bool,
    #[optional] pub primary_culture: String,
    #[optional] pub dominant_culture: String,
    #[repeated] pub accepted_culture: Vec<String>,
    #[optional] pub religion: IdRef<Religion>,
    #[optional] pub secondary_religion: IdRef<Religion>,
    #[optional] pub religious_school: String,
    #[optional] pub dominant_religion: IdRef<Religion>,
    #[optional] pub fervor: (),
    #[optional] pub technology_group: String,
    #[optional] pub liberty_desire: FixedPoint,
    #[repeated] pub temporary_liberty_desire: Vec<()>,
    #[optional] pub unit_type: String,
    pub technology: (),
    #[repeated] pub estate: Vec<()>,
    #[repeated] pub faction: Vec<()>,
    #[optional] pub top_faction: i32,
    #[repeated] pub rival: Vec<()>,
    pub highest_possible_fort: i32,
    pub highest_possible_fort_building: String,
    pub transfer_home_bonus: FixedPoint,
    #[repeated] pub enemy: Vec<CountryRef>,
    #[optional] pub overlord: CountryRef,
    #[optional] pub colonial_parent: CountryRef,
    #[repeated] pub gave_access: Vec<CountryRef>,
    #[optional] pub rebel_threat: i32,
    #[optional] pub goldtype: i32,
    #[optional] pub num_of_war_reparations: i32,
    #[repeated] pub our_spy_network: Vec<CountryRef>,
    #[repeated] pub their_spy_network: Vec<CountryRef>,
    #[optional] pub federation_leader: CountryRef,
    #[repeated] pub federation_friends: Vec<CountryRef>,
    #[repeated] pub coalition_against_us: Vec<CountryRef>,
    #[optional] pub coalition_target: CountryRef,
    #[optional] pub coalition_date: Date,
    #[repeated] pub preferred_coalition_against_us: Vec<CountryRef>,
    #[optional] pub preferred_coalition_target: CountryRef,
    #[optional] pub preferred_coalition_score: FixedPoint,
    #[optional] pub excommunicated: bool,
    #[optional] pub luck: bool,
    #[optional] pub convert: bool,
    #[optional] pub new_monarch: bool,
    #[optional] pub is_at_war: bool,
    #[optional] pub at_war_with_other_religious_group: bool,
    #[repeated] pub effective_score_impact: Vec<()>,
    #[optional] pub last_election: Date,
    #[optional] pub delayed_treasure: FixedPoint,
    #[optional] pub current_power_projection: FixedPoint,
    #[optional] pub great_power_score: FixedPoint,
    #[repeated] pub power_projection: Vec<()>,
    #[optional] pub num_of_trade_embargos: i32,
    #[optional] pub num_of_non_rival_trade_embargos: i32,
    #[optional] pub preferred_emperor: CountryRef,
    #[optional] pub is_elector: bool,
    #[optional] pub last_hre_vote: Date,
    pub navy_strength: f64,
    #[optional] pub parliament: (),
    #[optional] pub total_war_worth: i32,
    #[optional] pub num_of_rebel_controlled_provinces: i32,
    #[optional] pub num_of_revolts: i32,
    #[optional] pub num_of_rebel_armies: i32,
    #[optional] pub num_owned_home_cores: i32,
    #[optional] pub non_overseas_development: FixedPoint,
    #[optional] pub num_of_controlled_cities: i32,
    #[optional] pub num_of_ports: i32,
    #[optional] pub num_of_non_cores: i32,
    #[optional] pub num_of_core_ports: i32,
    #[optional] pub num_of_total_ports: i32,
    #[optional] pub num_of_cardinals: i32,
    #[optional] pub num_of_mercenaries: i32,
    #[optional] pub num_of_regulars: i32,
    #[optional] pub num_of_banners: i32,
    #[optional] pub num_of_rajput: i32,
    #[optional] pub num_of_streltsy: i32,
    #[optional] pub num_of_cities: i32,
    #[optional] pub num_of_provinces_in_states: i32,
    #[optional] pub num_of_provinces_in_territories: i32,
    #[optional] pub num_of_colonies: i32,
    #[optional] pub cached_colonies: i32,
    #[optional] pub forts: i32,
    #[optional] pub tribute_type: i32,
    #[optional] pub num_of_allies: i32,
    #[optional] pub num_of_royal_marriages: i32,
    #[optional] pub num_of_subjects: i32,
    #[optional] pub num_of_heathen_provs: i32,
    #[optional] pub num_of_heretic_provs: i32,
    #[optional] pub inland_sea_ratio: FixedPoint,
    #[optional] pub average_unrest: FixedPoint,
    #[optional] pub average_effective_unrest: FixedPoint,
    #[optional] pub average_autonomy: FixedPoint,
    #[optional] pub average_autonomy_above_min: FixedPoint,
    #[optional] pub average_home_autonomy: FixedPoint,
    #[optional] pub friend_tags: Vec<CountryRef>,
    #[optional] pub num_of_buildings_indexed: (),
    #[optional] pub num_of_buildings_under_construction_indexed: (),
    #[optional] pub produced_goods_value: Vec<FixedPoint>,
    #[optional] pub num_of_goods_produced: Vec<i32>,
    #[optional] pub traded: Vec<FixedPoint>,
    #[optional] pub num_of_religions_indexed: (),
    #[optional] pub num_of_religions_dev: (),
    #[optional] pub num_of_leaders: [i32; 4],
    #[optional] pub num_of_leaders_with_traits: [i32; 4],
    #[optional] pub num_of_free_leaders: [i32; 4],
    #[optional] pub num_of_subject_count_indexed: (),
    #[optional] pub border_pct: (),
    #[optional] pub border_sit: (),
    #[optional] pub border_provinces: Vec<i32>,
    #[optional] pub neighbours: Vec<CountryRef>,
    #[optional] pub home_neighbours: Vec<CountryRef>,
    #[optional] pub core_neighbours: Vec<CountryRef>,
    #[optional] pub call_to_arms_friends: Vec<CountryRef>,
    #[optional] pub subjects: Vec<CountryRef>,
    #[optional] pub allies: Vec<CountryRef>,
    #[optional] pub extended_allies: Vec<CountryRef>,
    #[optional] pub guarantees: Vec<CountryRef>,
    #[optional] pub coalition_friends: Vec<CountryRef>,
    #[optional] pub warnings: Vec<CountryRef>,
    #[optional] pub current_at_war_with: Vec<CountryRef>,
    #[optional] pub current_war_allies: Vec<CountryRef>,
    #[optional] pub trade_embargoed_by: Vec<CountryRef>,
    #[optional] pub trade_embargoes: Vec<CountryRef>,
    #[optional] pub transfer_trade_power_to: Vec<CountryRef>,
    #[optional] pub transfer_trade_power_from: Vec<CountryRef>,
    pub score_rating: [FixedPoint; 3],
    pub score_rank: [i32; 3],
    pub age_score: [FixedPoint; 4],
    pub vc_age_score: [FixedPoint; 4],
    pub score_place: i32,
    pub prestige: FixedPoint,
    pub stability: FixedPoint,
    pub treasury: FixedPoint,
    pub estimated_monthly_income: FixedPoint,
    #[optional] pub inflation: FixedPoint,
    #[optional] pub inflation_history: Vec<FixedPoint>,
    pub opinion_cache: Vec<i32>,
    pub under_construction: Vec<i32>,
    pub under_construction_queued: Vec<i32>,
    pub total_count: Vec<i32>,
    #[optional] pub owned_provinces: Vec<i32>,
    #[optional] pub controlled_provinces: Vec<i32>,
    #[optional] pub core_provinces: Vec<i32>,
    #[optional] pub claim_provinces: Vec<i32>,
    pub idea_may_cache: Vec<i32>,
    pub update_opinion_cache: bool,
    pub needs_refresh: bool,
    pub casus_bellis_refresh: bool,
    pub needs_rebel_unit_refresh: bool,
    #[optional] pub rebels_in_country: Vec<i32>,
    #[optional] pub war_exhaustion: FixedPoint,
    #[optional] pub monthly_war_exhaustion: FixedPoint,
    #[optional] pub last_bankrupt: Date,
    pub can_take_wartaxes: bool,
    pub land_maintenance: FixedPoint,
    pub naval_maintenance: FixedPoint,
    pub colonial_maintenance: FixedPoint,
    pub missionary_maintenance: FixedPoint,
    #[optional] pub army_tradition: FixedPoint,
    #[optional] pub navy_tradition: FixedPoint,
    pub last_war_ended: Date,
    pub num_uncontested_cores: i32,
    pub ledger: (),
    pub loan_size: i32,
    pub estimated_loan: FixedPoint,
    #[repeated] pub loan: Vec<()>,
    #[optional] pub religious_unity: FixedPoint,
    #[optional] pub republican_tradition: FixedPoint,
    #[optional] pub devotion: FixedPoint,
    #[optional] pub meritocracy: FixedPoint,
    #[optional] pub piety: FixedPoint,
    #[optional] pub recovery_motivation: FixedPoint,
    #[optional] pub papal_influence: FixedPoint,
    #[optional] pub blessing: String,
    #[optional] pub corruption: FixedPoint,
    #[optional] pub root_out_corruption_slider: FixedPoint,
    #[optional] pub doom: FixedPoint,
    #[optional] pub authority: FixedPoint,
    #[optional] pub patriarch_authority: FixedPoint,
    #[optional] pub personal_deity: String,
    #[optional] pub fetishist_cult: String,
    #[optional] pub unlock_cult: Vec<String>,
    #[optional] pub legitimacy: FixedPoint,
    #[optional] pub horde_unity: FixedPoint,
    pub mercantilism: FixedPoint,
    pub splendor: FixedPoint,
    #[optional] pub army_professionalism: f64,
    #[optional] pub max_historic_army_professionalism: f64,
    #[optional] pub church: (),
    pub active_idea_groups: HashMap<String, i32>,
    #[optional] pub active_religious_reform: (),
    #[optional] pub active_native_advancement: (),
    #[repeated] pub advisor: Vec<()>,
    #[optional] pub government: (),
    #[optional] pub merchants: (),
    #[optional] pub missionaries: (),
    #[optional] pub diplomats: (),
    #[repeated] pub modifier: Vec<()>,
    pub manpower: FixedPoint,
    pub max_manpower: FixedPoint,
    #[optional] pub sailors: FixedPoint,
    pub max_sailors: FixedPoint,
    #[optional] pub sub_unit: (),
    pub num_of_captured_ships_with_boarding_doctrine: i32,
    #[optional] pub overextension_percentage: FixedPoint,
    #[repeated] pub army: Vec<()>,
    #[repeated] pub navy: Vec<()>,
    pub active_relations: (),
    #[repeated] pub leader: Vec<()>,
    pub decision_seed: i32,
    #[optional] pub monarch: (),
    #[optional] pub heir: (),
    #[optional] pub queen: (),
    #[optional] pub original_dynasty: String,
    pub num_of_consorts: i32,
    #[optional] pub is_great_power: bool,
    #[optional] pub wants_to_be_great_power: bool,
    #[optional] pub wants_to_be_great_power_next: bool,
    pub inauguration: Date,
    #[optional] pub last_migration: Date,
    #[repeated] pub previous_monarch: Vec<()>,
    pub ai: (),
    #[optional] pub assigned_estates: bool,
    #[optional] pub historical_friends: Vec<CountryRef>,
    #[optional] pub historical_rivals: Vec<CountryRef>,
    #[optional] pub traded_bonus: Vec<i32>,
    pub powers: Vec<i32>,
    #[optional] pub interesting_countries: Vec<i32>,
    #[repeated] pub delayed_event: Vec<()>,
    #[optional] pub blockaded_percent: FixedPoint,
    #[optional] pub native_policy: i32,
    #[optional] pub anti_nation_ruining_end_date: Date,
    #[optional] pub spy_propensity: FixedPoint,
    pub losses: (),
    #[optional] pub adm_spent_indexed: (),
    #[optional] pub dip_spent_indexed: (),
    #[optional] pub mil_spent_indexed: (),
    #[optional] pub debase_recharge_need: i32,
    #[optional] pub influenced_by: CountryRef,
    #[optional] pub mothballed_forts: Vec<i32>,
    pub innovativeness: FixedPoint,
    #[optional] pub completed_missions: Vec<String>,
    pub historic_stats_cache: (),
    pub country_missions: (),
    #[optional] pub government_reform_progress: FixedPoint,

    // Unsorted entries:
    #[optional] pub call_for_peace: FixedPoint,
    #[optional] pub current_icon: i32,
    #[optional] pub forced_break_alliance_date: Date,
    #[optional] pub has_friendly_reformation_center: bool,
    #[optional] pub has_privateers: bool,
    #[optional] pub icon_start_date: Date,
    #[optional] pub last_conversion: Date,
    #[optional] pub last_conversion_secondary: Date,
    #[optional] pub last_sacrifice: Date,
    #[optional] pub last_sold_province: Date,
    #[optional] pub naval_doctrine: String,
    #[optional] pub num_of_janissaries: i32,
    #[optional] pub num_of_overseas: i32,
    #[optional] pub num_ships_privateering: i32,
    #[optional] pub saved_names: (),
    #[optional] pub tribal_allegiance: FixedPoint,
    #[optional] pub wartax: i32,
    #[optional] pub condottieri_client: Vec<CountryRef>,
    #[optional] pub hired_condottieri_from: Vec<CountryRef>,
    #[repeated] pub previous_country_tags: Vec<CountryRef>,
    #[optional] pub colonists: (),
    #[optional] pub cooldowns: (),
    #[optional] pub disaster_started: Vec<i32>,
    #[optional] pub disaster_progress: Vec<FixedPoint>,
    #[repeated] pub ignore_decision: Vec<String>,
    #[optional] pub colonial_core: i32,
    #[optional] pub golden_era_date: Date,
    #[optional] pub diplomacy: (), // Or is this repeated?
    #[optional] pub harmonization_progress: FixedPoint,
    #[optional] pub harmonizing_with_religion: IdRef<Religion>,
    #[optional] pub ai_condottieri_malus_until: Date,
    #[optional] pub num_of_independence_supporters: i32,
    #[optional] pub ai_condottieri_dont_send_until: (),
    #[optional] pub active_disaster: String,
    #[optional] pub subject_interactions: (),
    #[optional] pub support_independence: (),
}

#[derive(ParadoxParse, Default)]
pub struct Province {
    #[optional] pub flags: HashMap<String, Date>,
    pub name: String,
    #[optional] pub territorial_core: CountryRef,
    #[optional] pub owner: CountryRef,
    #[optional] pub controller: CountryRef,
    #[optional] pub previous_controller: CountryRef,
    #[optional] pub original_coloniser: CountryRef,
    #[optional] pub seat_in_parliament: (),
    #[optional] pub occupying_rebel_faction: (),
    pub institutions: Vec<FixedPoint>,
    #[optional] pub estate: i32,
    #[optional] pub last_estate_grant: Date,
    #[optional] pub cores: Vec<CountryRef>,
    #[optional] pub claims: Vec<CountryRef>,
    #[optional] pub trade: String,
    #[optional] pub unit: (),
    #[optional] pub spy_actions: (),
    #[optional] pub original_culture: String,
    #[optional] pub culture: String,
    #[optional] pub religion: IdRef<Religion>,
    #[optional] pub original_religion: IdRef<Religion>,
    #[optional] pub capital: String,
    #[optional] pub is_city: bool,
    #[optional] pub colonysize: FixedPoint,
    #[optional] pub native_size_before_migration: FixedPoint,
    #[optional] pub garrison: FixedPoint,
    #[optional] pub siege: FixedPoint,
    #[optional] pub base_tax: FixedPoint,
    #[optional] pub original_tax: FixedPoint,
    #[optional] pub base_production: FixedPoint,
    #[optional] pub base_manpower: FixedPoint,
    #[optional] pub unrest: FixedPoint,
    #[optional] pub likely_rebels: String,
    #[optional] pub hre: bool,
    pub trade_goods: String,
    #[optional] pub devastation: FixedPoint,
    #[optional] pub local_autonomy: FixedPoint,
    #[optional] pub ub: bool,
    #[optional] pub blockade: bool,
    #[optional] pub blockade_efficiency: FixedPoint,
    #[optional] pub buildings: (),
    #[optional] pub building_builders: (),
    #[optional] pub history: (),
    pub patrol: i32,
    #[optional] pub discovery_dates2: (),
    #[optional] pub discovery_religion_dates2: (),
    pub discovered_by: Vec<CountryRef>,
    #[optional] pub native_size: FixedPoint,
    #[optional] pub native_hostileness: i32,
    #[optional] pub native_ferocity: i32,
    #[optional] pub improve_count: i32,
    #[optional] pub nationalism: i32,
    #[optional] pub winter: i32,
    #[optional] pub previous_winter: i32,
    #[repeated] pub modifier: Vec<()>,
    #[repeated] pub triggered_modifier: Vec<String>,
    #[optional] pub applied_triggered_modifier: String,
    #[repeated] pub diplomacy_construction: Vec<()>,
    #[repeated] pub merchant_construction: Vec<()>,
    #[repeated] pub military_construction: Vec<()>,
    #[optional] pub building_construction: (),
    #[optional] pub build_core_construction: (),
    #[optional] pub colony_construction: (),
    #[optional] pub missionary_construction: (),
    #[optional] pub settlement_growth_construction: (),
    #[repeated] pub fort_influencing: Vec<()>,
    #[optional] pub mothball_command: bool,
    #[optional] pub trade_power: FixedPoint,
    #[optional] pub last_native_uprising: Date,
    #[optional] pub missionary_progress: FixedPoint,
    #[optional] pub rebel_faction: (),
    #[optional] pub loot_remaining: FixedPoint,
    #[optional] pub last_looted: Date,
    #[optional] pub last_razed: Date,
    #[optional] pub last_razed_by: CountryRef,
    #[optional] pub hostile_core_creation_cost: FixedPoint,
    #[optional] pub hostile_core_creation_tag: CountryRef,
    #[optional] pub hostile_core_creation_desc: String,
    #[optional] pub center_of_trade: i32,

    // Unordered:
    #[optional] pub hre_liberated: bool,
    #[optional] pub former_native_size: FixedPoint,
    #[optional] pub latent_trade_goods: (),
    #[optional] pub active_trade_company: bool,
    #[optional] pub center_of_religion: bool,
}

#[derive(ParadoxParse, Default)]
pub struct Statistics {
    #[repeated] pub ledger_data: Vec<LedgerData>,
}

#[derive(ParadoxParse, Default)]
pub struct LedgerData {
    pub name: CountryRef,
    #[optional] pub data: HashMap<String, i32>,
}
