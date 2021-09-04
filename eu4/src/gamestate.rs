use crate::{Eu4Atom, GameData, Modifiers};
use paradox::{Date, FixedPoint, ParadoxParse};
use std::collections::HashMap;

//type CountryRef = IdRef<crate::Country>;
type CountryRef = Eu4Atom;

#[derive(ParadoxParse, Default)]
pub struct Gamestate {
    #[optional] pub players_countries: (),
    pub gameplaysettings: (),
    pub speed: i32,
    pub multiplayer_random_seed: u32,
    pub multiplayer_random_count: i32,
    pub current_age: Eu4Atom, // XXX
    pub next_age_progress: FixedPoint,
    pub id_counters: Vec<u32>,
    pub unit: i32,
    pub unit_template_id: i32,
    pub flags: HashMap<Eu4Atom, Date>,
    pub start_date: Date,
    pub map_area_data: HashMap<Eu4Atom, ()>,
    pub total_military_power: f64,
    pub average_military_power: f64,
    pub institution_origin: Vec<i32>, // XXX: ProvinceRef
    pub institutions: Vec<i32>,
    pub institutions_penalties: Vec<FixedPoint>,
    pub trade: Trade,
    #[repeated] pub unit_templates: Vec<()>,
    pub production_leader_tag: Vec<CountryRef>,
    pub tradegoods_total_produced: Vec<FixedPoint>,
    pub change_price: HashMap<Eu4Atom, ()>,
    pub id: (),
    pub dynasty: (),
    #[repeated] pub rebel_faction: Vec<()>,
    pub great_powers: (),
    pub empire: (),
    pub celestial_empire: (),
    pub hre_leagues_status: i32,
    pub hre_religion_status: i32,
    #[repeated] pub trade_league: Vec<TradeLeague>,
    pub religions: HashMap<Eu4Atom, ()>,
    pub religion_instance_data: HashMap<Eu4Atom, ()>,
    pub fired_events: (),
    pub pending_events: (),
    pub provinces: HashMap<Eu4Atom, Province>,
    pub countries: HashMap<CountryRef, Country>,
    pub active_advisors: HashMap<Eu4Atom, ()>,
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
    #[optional] pub completed_achievements: (),
    pub unit_manager: (),
    pub trade_company_manager: (),
    pub tech_level_dates: (), // it's a [(String, Date); 3]
    pub idea_dates: HashMap<Eu4Atom, Date>,
    #[repeated] pub checksum: Vec<String>,
    #[optional] pub player: CountryRef,
}

#[derive(ParadoxParse, Default)]
pub struct Trade {
    #[repeated] pub node: Vec<TradeNode>,
}

#[derive(ParadoxParse, Default)]
pub struct TradeIncoming {
    pub add: FixedPoint,
    pub value: FixedPoint,
    pub from: i32,
}

#[derive(ParadoxParse, Default)]
pub struct TradeNode {
    pub definitions: Eu4Atom,
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
    #[repeated] pub incoming: Vec<TradeIncoming>,
    pub trade_goods_size: Vec<FixedPoint>,
    #[optional] pub top_provinces: Vec<CountryRef>,
    #[optional] pub top_provinces_values: Vec<FixedPoint>,
    #[optional] pub top_power: Vec<CountryRef>,
    #[optional] pub top_power_values: Vec<FixedPoint>,
    #[optional] pub trade_company_region: bool,
    pub most_recent_treasure_ship_passage: Date,
    #[collect] pub country_info: Vec<(Eu4Atom, CountryTradeNode)>,
}

#[derive(ParadoxParse, Default)]
pub struct CountryTradeNodeModifier {
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
    #[optional] pub has_trader: bool,
    #[optional] pub has_capital: bool,
    #[optional] pub light_ship: i32,
    #[optional] pub t_in: FixedPoint,
    #[optional] pub t_from: HashMap<CountryRef, FixedPoint>,
    #[optional] pub t_out: FixedPoint,
    #[optional] pub t_to: HashMap<CountryRef, FixedPoint>,
    #[optional] pub trading_policy: Eu4Atom,
    #[optional] pub trading_policy_date: Date,
    #[repeated] pub modifier: Vec<()>,
    #[optional] pub privateer_mission: FixedPoint,
    #[optional] pub privateer_money: FixedPoint,
}

#[derive(ParadoxParse, Default)]
pub struct AppliedModifiers {
    pub modifier: Eu4Atom,
    pub date: Date,
    #[optional] pub hidden: bool,
    #[optional] pub ruler_modifier: bool,
}

#[derive(ParadoxParse, Default)]
pub struct ActivePolicy {
    pub policy: Eu4Atom,
    pub date: Date,
}

#[derive(ParadoxParse, Default)]
pub struct HreInfo {
    pub emperor: CountryRef,
    pub imperial_influence: FixedPoint,
    #[repeated] pub old_emperor: Vec<()>,
    #[repeated] pub passed_reform: Vec<Eu4Atom>,
    pub continent: u32,
    pub imperial_ban_allowed: bool,
    pub internal_hre_cb: bool,
    pub hre_inheritable: bool,
    pub allows_female_emperor: bool,
    pub emperor_has_revoked: bool,
    pub electors: Vec<CountryRef>,
    pub emperor_previous_rank: u32,
    pub imperial_realm_war: bool,
    #[repeated] pub previous_incident: Vec<()>,
}

#[derive(ParadoxParse, Default)]
pub struct Country {
    #[optional] pub human: bool,
    #[optional] pub was_player: bool,
    #[optional] pub has_set_government_name: bool,
    pub government_rank: i32,
    #[optional] pub government_name: Eu4Atom,
    pub subject_focus: i32,
    pub trade_mission: FixedPoint,
    pub blockade_mission: FixedPoint,
    pub continent: Vec<i32>,
    #[optional] pub national_focus: Eu4Atom,
    pub institutions: Vec<i32>,
    #[optional] pub technology_cost: FixedPoint,
    #[optional] pub num_of_age_objectives: i32,
    #[repeated] pub active_age_ability: Vec<Eu4Atom>,
    #[optional] pub last_focus_move: Date,
    #[optional] pub last_sent_alliance_offer: Date,
    #[optional] pub history: (),
    #[optional] pub flags: HashMap<Eu4Atom, Date>,
    #[optional] pub hidden_flags: HashMap<Eu4Atom, Date>,
    #[optional] pub variables: HashMap<Eu4Atom, FixedPoint>,
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
    #[optional] pub potential_incidents: Vec<Eu4Atom>,
    #[optional] pub active_incidents: Vec<Eu4Atom>,
    #[optional] pub past_incidents: Vec<Eu4Atom>,
    #[optional] pub incident_variables: HashMap<Eu4Atom, FixedPoint>,
    #[optional] pub harmony: FixedPoint,
    #[optional] pub harmonized_religions: Vec<i32>,
    pub initialized_rivals: bool,
    pub recalculate_strategy: bool,
    pub colors: (),
    #[optional] pub name: String,
    #[optional] pub adjective: String,
    pub dirty_colony: bool,
    #[optional] pub primary_culture: Eu4Atom,
    #[optional] pub dominant_culture: Eu4Atom,
    #[repeated] pub accepted_culture: Vec<Eu4Atom>,
    #[optional] pub religion: Eu4Atom,
    #[optional] pub secondary_religion: Eu4Atom,
    #[optional] pub religious_school: Eu4Atom,
    #[optional] pub dominant_religion: Eu4Atom,
    #[optional] pub fervor: (),
    #[optional] pub technology_group: Eu4Atom,
    #[optional] pub liberty_desire: FixedPoint,
    #[repeated] pub temporary_liberty_desire: Vec<()>,
    #[optional] pub unit_type: Eu4Atom,
    pub technology: (),
    #[repeated] pub estate: Vec<()>,
    #[repeated] pub faction: Vec<()>,
    #[optional] pub top_faction: i32,
    #[repeated] pub rival: Vec<()>,
    pub highest_possible_fort: i32,
    pub highest_possible_fort_building: Eu4Atom,
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
    #[optional] pub blessing: Eu4Atom,
    #[optional] pub corruption: FixedPoint,
    #[optional] pub root_out_corruption_slider: FixedPoint,
    #[optional] pub doom: FixedPoint,
    #[optional] pub authority: FixedPoint,
    #[optional] pub patriarch_authority: FixedPoint,
    #[optional] pub personal_deity: Eu4Atom,
    #[optional] pub fetishist_cult: Eu4Atom,
    #[optional] pub unlock_cult: Vec<Eu4Atom>,
    #[optional] pub legitimacy: FixedPoint,
    #[optional] pub horde_unity: FixedPoint,
    pub mercantilism: FixedPoint,
    pub splendor: FixedPoint,
    #[optional] pub army_professionalism: f64,
    #[optional] pub max_historic_army_professionalism: f64,
    #[optional] pub church: (),
    pub active_idea_groups: HashMap<Eu4Atom, i32>,
    #[optional] pub active_religious_reform: (),
    #[optional] pub active_native_advancement: (),
    #[repeated] pub advisor: Vec<()>,
    #[optional] pub government: (),
    #[optional] pub merchants: (),
    #[optional] pub missionaries: (),
    #[optional] pub diplomats: (),
    #[repeated] pub modifier: Vec<AppliedModifiers>,
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
    #[optional] pub completed_missions: Vec<Eu4Atom>,
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
    #[optional] pub naval_doctrine: Eu4Atom,
    #[optional] pub num_of_janissaries: i32,
    #[optional] pub num_of_overseas: i32,
    #[optional] pub num_ships_privateering: i32,
    #[optional] pub saved_names: (),
    #[optional] pub tribal_allegiance: FixedPoint,
    #[optional] pub wartax: bool,
    #[optional] pub condottieri_client: Vec<CountryRef>,
    #[optional] pub hired_condottieri_from: Vec<CountryRef>,
    #[repeated] pub previous_country_tags: Vec<CountryRef>,
    #[optional] pub colonists: (),
    #[optional] pub cooldowns: (),
    #[optional] pub disaster_started: Vec<i32>,
    #[optional] pub disaster_progress: Vec<FixedPoint>,
    #[repeated] pub ignore_decision: Vec<Eu4Atom>,
    #[optional] pub colonial_core: i32,
    #[optional] pub golden_era_date: Date,
    #[optional] pub diplomacy: (), // Or is this repeated?
    #[optional] pub harmonization_progress: FixedPoint,
    #[optional] pub harmonizing_with_religion: Eu4Atom,
    #[optional] pub ai_condottieri_malus_until: Date,
    #[optional] pub num_of_independence_supporters: i32,
    #[optional] pub ai_condottieri_dont_send_until: (),
    #[optional] pub active_disaster: Eu4Atom,
    #[optional] pub subject_interactions: (),
    #[optional] pub support_independence: (),
    #[optional] pub update_supply_range: bool,
    #[optional] pub absolutism: FixedPoint,
    #[optional] pub has_circumnavigated_world: bool,
    #[repeated] pub active_policy: Vec<ActivePolicy>,
    #[optional] pub mercenary_company: (),
    #[optional] pub used_governing_capacity: FixedPoint,
    #[optional] pub hre_vote: i32,
    #[optional] pub force_converted: Eu4Atom,
    #[optional] pub interactions_last_used: (),
    #[optional] pub last_sent_peace_offer_date: Date,
    #[optional] pub num_expanded_administration: i32,
    #[optional] pub historical_council: i32,
    #[optional] pub council_active: i32,
    #[optional] pub pillaged_capital_state: (),
    #[optional] pub join_hre: Date,
}

impl Country {
    pub fn get_modifiers(&self, data: &GameData,
                         gamestate: &Gamestate, tag: &Eu4Atom) -> Modifiers {
        let mut mods = Modifiers::default();
        // Static modifiers
        let static_mod = &data.static_modifiers;
        // XXX: patriarch_authority_global
        macro_rules! apply_static {
            ($label:ident) => {
                let atom = Eu4Atom::from(stringify!($label));
                let modifier = &static_mod[&atom].modifiers;
                mods.add_modifiers(modifier);
            };
            (scaled $label:ident) => {
                let atom = Eu4Atom::from(stringify!($label));
                let modifier = &static_mod[&atom].modifiers;
                mods.add_scaled_modifiers(modifier, self.$label);
            };
            (scaled 100 $label:ident) => {
                let atom = Eu4Atom::from(stringify!($label));
                let modifier = &static_mod[&atom].modifiers;
                mods.add_scaled_modifiers(modifier, self.$label / 100.into());
            };
            (+/- $label:ident) => {
                let value = self.$label;
                let (atom, scale) = if value < FixedPoint::ZERO {
                    (Eu4Atom::from(concat!("negative_", stringify!($label))),
                        -value)
                } else {
                    (Eu4Atom::from(concat!("positive_", stringify!($label))),
                        value)
                };
                let modifier = &static_mod[&atom].modifiers;
                mods.add_scaled_modifiers(modifier, scale);
            };
            (scaled $label:ident * $e:expr) => {
                let atom = Eu4Atom::from(stringify!($label));
                let modifier = &static_mod[&atom].modifiers;
                mods.add_scaled_modifiers(modifier, $e);
            };
        }
        mods.add_modifiers(&static_mod[&Eu4Atom::from("base_values")].modifiers);
        // XXX: war_taxes
        apply_static!(scaled stability);
        apply_static!(+/- stability);
        // XXX: privateering
        // XXX: positive/negative mandate
        apply_static!(scaled inflation);
        // XXX: bankruptcy
        // XXX: war, peace, unconditional_surrender, call_for_peace
        apply_static!(scaled war_exhaustion);
        apply_static!(scaled 100 doom);
        apply_static!(scaled 100 authority);
        // XXX: regency_council, trade_efficiency, production_efficiency
        // XXX: trade_refusal, mercantilism
        apply_static!(scaled 100 army_tradition);
        apply_static!(scaled 100 navy_tradition);
        apply_static!(+/- piety);
        // XXX: DoF, emperor/HRE logic
        // XXX: num of marriages/provinces, development
        apply_static!(scaled 100 tribal_allegiance);
        let fp_50 : FixedPoint = 50.into();
        apply_static!(scaled legitimacy * self.legitimacy - fp_50);
        apply_static!(scaled horde_unity * self.horde_unity - fp_50);
        apply_static!(scaled devotion * self.devotion - fp_50);
        apply_static!(scaled meritocracy * self.meritocracy - fp_50);
        if self.meritocracy < fp_50 {
            apply_static!(scaled low_meritocracy * fp_50 - self.meritocracy);
        }
        apply_static!(scaled 100 corruption);
        // XXX: root out corruption, recovery_motivation, militarized_socierty
        // XXX: luck, OE
        apply_static!(scaled 100 prestige);
        // XXX: parliament, republican tradition
        // XXX: bunch of stuff (curia_controller - streltsy)
        apply_static!(scaled power_projection *
                      self.current_power_projection / 100.into());
        if self.current_power_projection >= 25.into() {
            apply_static!(power_projection_25);
        }
        // XXX: karma, natives, harmony
        apply_static!(scaled 100 innovativeness);

        // More complex static modifiers
        for trade_league in &gamestate.trade_league {
            if &trade_league.members[0] == tag {
                mods.add_scaled_modifiers(
                    &static_mod[&Eu4Atom::from("scaled_trade_league_leader")].modifiers,
                    (trade_league.members.len() as i32).into());
            }
            if trade_league.members.contains(tag) {
                apply_static!(in_trade_league);
                break;
            }
        }

        // XXX: more static modifiers
        for (idea_group_name, &idea_count) in &self.active_idea_groups {
            data.idea_groups[idea_group_name]
                .add_idea_modifiers(idea_count, &mut mods);
        }
        // XXX: techs
        // XXX: religion (+ancestors, cults, etc.)
        for modifier in &self.modifier {
            for map in &[&data.event_modifiers, &data.static_modifiers] {
                if let Some(val) = map.get(&modifier.modifier) {
                    mods.add_modifiers(&val.modifiers);
                    break;
                }
            }
        }
        for policy in &self.active_policy {
            mods.add_modifiers(&data.policies[&policy.policy].modifiers);
        }
        // XXX: tradegoods
        // XXX: advisors
        // XXX: government rank, government reforms
        // XXX: monarch whatevers
        // XXX: naval doctrine
        // XXX: great projects
        // XXX: estates, factions
        // XXX: ruler personality
        // XXX: age bonus
        // XXX: disasters
        mods
    }
}

#[derive(ParadoxParse, Default)]
pub struct Province {
    #[optional] pub flags: HashMap<Eu4Atom, Date>,
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
    #[optional] pub trade: Eu4Atom, // IdRef<crate::TradeNode>,
    #[optional] pub unit: (),
    #[optional] pub spy_actions: (),
    #[optional] pub original_culture: Eu4Atom,
    #[optional] pub culture: Eu4Atom,
    #[optional] pub religion: Eu4Atom,
    #[optional] pub original_religion: Eu4Atom,
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
    #[optional] pub likely_rebels: Eu4Atom,
    #[optional] pub hre: bool,
    pub trade_goods: Eu4Atom,
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
    #[repeated] pub triggered_modifier: Vec<Eu4Atom>,
    #[optional] pub applied_triggered_modifier: Eu4Atom,
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
    #[optional] pub native_culture: Eu4Atom,
    #[optional] pub country_improve_count: (),
}

#[derive(ParadoxParse, Default)]
pub struct TradeLeague {
    id: i32,
    members: Vec<CountryRef>
}

#[derive(ParadoxParse, Default)]
pub struct Statistics {
    #[repeated] pub ledger_data: Vec<LedgerData>,
}

#[derive(ParadoxParse, Default)]
pub struct LedgerData {
    pub name: CountryRef,
    #[optional] pub data: HashMap<Eu4Atom, i32>,
}
