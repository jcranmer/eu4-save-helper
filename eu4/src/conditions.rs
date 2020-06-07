use paradox::{FixedPoint, IdRef, ParadoxParse};
use crate::{AdvisorType, Continent, Country, Culture, CultureGroup, IdeaGroup, Religion, TradeGood};

#[derive(Debug)]
pub enum Variable<T> {
    Value(T),
    Scope(String),
    Substitution(String),
}

impl <T: Default> Default for Variable<T> {
    fn default() -> Self {
        Self::Value(T::default())
    }
}

impl <T: ParadoxParse + Default> ParadoxParse for Variable<T> {
    fn read_from(&mut self, parser: &mut paradox::Parser, value: paradox::Token) -> Result<(), paradox::ParseError> {
        if let Ok(s) = value.try_to_string() {
            if s.contains("$") {
                *self = Self::Substitution(s.into());
                return Ok(());
            }
            match s {
                "owner" | "controller" | "emperor" | "ROOT" | "FROM" | "PREV" |
                    "THIS" | "CAPITAL" => {
                        *self = Self::Scope(s.into());
                        return Ok(());
                    },
                _ => {},
            }
        }
        let mut v : T = Default::default();
        v.read_from(parser, value)?;
        *self = Self::Value(v);
        Ok(())
    }
}

paradox::condition_list!{
    condition(Country, <advisor_type: AdvisorType>, i32);
    condition(Country, <idea_group: IdeaGroup>, i32);
    condition(Country, <trade_good: TradeGood>, i32);
    condition(Country, absolutism, FixedPoint);
    condition(Country, accepted_culture, IdRef<Culture>);
    condition(Country, adm, i32);
    condition(Country, adm_power, i32);
    condition(Country, adm_tech, i32);
    condition(Country, advisor, IdRef<AdvisorType>);
    condition(Country, ai, bool);
    condition(Country, army_professionalism, FixedPoint);
    condition(Country, army_tradition, FixedPoint);
    condition(Country, can_heir_be_child_of_consort, bool);
    condition(Country, consort_adm, i32);
    condition(Country, consort_age, i32);
    condition(Country, consort_dip, i32);
    condition(Country, consort_has_personality, String);
    condition(Country, consort_mil, i32);
    condition(Country, corruption, FixedPoint);
    condition(Country, culture, IdRef<Culture>);
    condition(Country, culture_group, IdRef<CultureGroup>);
    condition(Country, dip, i32);
    condition(Country, dip_power, i32);
    condition(Country, dip_tech, i32);
    condition(Country, dominant_religion, IdRef<Religion>);
    condition(Country, dynasty, String);
    condition(Country, employed_advisor, ()); // XXX -- complex clauses...
    condition(Country, estate_influence, ()); // XXX -- complex clauses...
    condition(Country, estate_loyalty, ()); // XXX -- complex clauses...
    condition(Country, exists, IdRef<Country>);
    condition(Country, faction_in_power, String);
    condition(Country, government, String);
    condition(Country, had_country_flag, ()); // XXX -- complex clauses...
    condition(Country, had_recent_war, i32);
    condition(Country, had_ruler_flag, ()); // XXX -- complex clauses...
    condition(Country, harmonization_progress, i32);
    condition(Country, has_advisor, bool);
    condition(Country, has_consort, bool);
    condition(Country, has_consort_flag, String);
    condition(Country, has_consort_regency, bool);
    condition(Country, has_country_flag, String);
    condition(Country, has_country_modifier, String);
    condition(Country, has_dlc, String);
    condition(Country, has_estate, String);
    condition(Country, has_estate_influence_modifier, ()); // XXX -- complex clauses...
    condition(Country, has_female_consort, bool);
    condition(Country, has_foreign_consort, bool);
    condition(Country, has_government_attribute, String);
    condition(Country, has_heir, bool);
    condition(Country, has_heir_flag, String);
    condition(Country, has_idea, String);
    condition(Country, has_idea_group, String);
    condition(Country, has_institution, String);
    condition(Country, has_leaders, ()); // XXX -- complex clauses...
    condition(Country, has_mission, String);
    condition(Country, has_opinion, ()); // XXX -- complex clauses...
    condition(Country, has_parliament, bool);
    condition(Country, has_reform, String);
    condition(Country, has_regency, bool);
    condition(Country, has_ruler_flag, String);
    condition(Country, has_ruler_modifier, String);
    condition(Country, has_spawned_rebels, String);
    condition(Country, heir_adm, i32);
    condition(Country, heir_age, i32);
    condition(Country, heir_claim, i32);
    condition(Country, heir_dip, i32);
    condition(Country, heir_has_personality, String);
    condition(Country, heir_mil, i32);
    condition(Country, imperial_mandate, FixedPoint);
    condition(Country, inflation, FixedPoint);
    condition(Country, is_at_war, bool);
    condition(Country, is_bankrupt, bool);
    condition(Country, is_client_nation, bool);
    condition(Country, is_colonial_nation, bool);
    condition(Country, is_crusade_target, bool);
    condition(Country, is_defender_of_faith, bool);
    condition(Country, is_dynamic_tag, bool);
    condition(Country, is_elector, bool);
    condition(Country, is_emperor, bool);
    condition(Country, is_emperor_of_china, bool);
    condition(Country, is_excommunicated, bool);
    condition(Country, is_federation_leader, bool);
    condition(Country, is_female, bool);
    condition(Country, is_force_converted, bool);
    condition(Country, is_former_colonial_nation, bool);
    condition(Country, is_great_power, bool);
    condition(Country, is_heir_leader, bool);
    condition(Country, is_in_coalition, bool);
    condition(Country, is_in_coalition_war, bool);
    condition(Country, is_in_deficit, bool);
    condition(Country, is_in_league_war, bool);
    condition(Country, is_in_trade_league, bool);
    condition(Country, is_league_leader, bool);
    condition(Country, is_lesser_in_union, bool);
    condition(Country, is_march, bool);
    condition(Country, is_monarch_leader, bool);
    condition(Country, is_nomad, bool);
    condition(Country, is_orangists_in_power, bool);
    condition(Country, is_overseas_subject, bool);
    condition(Country, is_papal_controller, bool);
    condition(Country, is_part_of_hre, bool);
    condition(Country, is_playing_custom_nation, bool);
    condition(Country, is_previous_papal_controller, bool);
    condition(Country, is_protectorate, bool);
    condition(Country, is_religion_enabled, IdRef<Religion>);
    condition(Country, is_religion_reformed, bool);
    condition(Country, is_revolution_target, bool);
    condition(Country, is_statists_in_power, bool);
    condition(Country, is_subject, bool);
    condition(Country, is_subject_of_type, String);
    condition(Country, is_trade_league_leader, bool);
    condition(Country, is_tribal, bool);
    condition(Country, is_vassal, bool);
    condition(Country, karma, FixedPoint);
    condition(Country, legitimacy, FixedPoint);
    condition(Country, liberty_desire, FixedPoint);
    condition(Country, manpower_percentage, FixedPoint);
    condition(Country, mercantilism, FixedPoint);
    condition(Country, meritocracy, FixedPoint);
    condition(Country, mil, i32);
    condition(Country, mil_power, i32);
    condition(Country, mil_tech, i32);
    condition(Country, monthly_income, FixedPoint);
    condition(Country, navy_size, u32);
    condition(Country, num_of_admirals, i32);
    condition(Country, num_of_admirals_with_traits, i32);
    condition(Country, num_of_allies, i32);
    condition(Country, num_of_artillery, i32);
    condition(Country, num_of_captured_ships_with_boarding_doctrine, i32);
    condition(Country, num_of_cardinals, i32);
    condition(Country, num_of_cavalry, i32);
    condition(Country, num_of_cities, i32);
    condition(Country, num_of_coalition_members, i32);
    condition(Country, num_of_colonies, i32);
    condition(Country, num_of_colonists, i32);
    condition(Country, num_of_conquistadors, i32);
    condition(Country, num_of_consorts, i32);
    condition(Country, num_of_continents, i32);
    condition(Country, num_of_cossacks, i32);
    condition(Country, num_of_custom_nations, i32);
    condition(Country, num_of_diplomatic_relations, i32);
    condition(Country, num_of_diplomats, i32);
    condition(Country, num_of_explorers, i32);
    condition(Country, num_of_free_diplomatic_relations, i32);
    condition(Country, num_of_galley, i32);
    condition(Country, num_of_generals, i32);
    condition(Country, num_of_generals_with_traits, i32);
    condition(Country, num_of_harmonized, i32);
    condition(Country, num_of_heavy_ship, i32);
    condition(Country, num_of_infantry, i32);
    condition(Country, num_of_light_ship, i32);
    condition(Country, num_of_loans, i32);
    condition(Country, num_of_marches, i32);
    condition(Country, num_of_mercenaries, i32);
    condition(Country, num_of_merchants, i32);
    condition(Country, num_of_missionaries, i32);
    condition(Country, num_of_owned_and_controlled_institutions, i32);
    condition(Country, num_of_ports, i32);
    condition(Country, num_of_ports_blockading, i32);
    condition(Country, num_of_powerful_estates, i32);
    condition(Country, num_of_protectorates, i32);
    condition(Country, num_of_provinces_in_states, i32);
    condition(Country, num_of_provinces_in_territories, i32);
    condition(Country, num_of_rebel_armies, i32);
    condition(Country, num_of_rebel_controlled_provinces, i32);
    condition(Country, num_of_revolts, i32);
    condition(Country, num_of_royal_marriages, i32);
    condition(Country, num_of_states, i32);
    condition(Country, num_of_streltsy, i32);
    condition(Country, num_of_strong_trade_companies, i32);
    condition(Country, num_of_subjects, i32);
    condition(Country, num_of_territories, i32);
    condition(Country, num_of_times_improved, i32);
    condition(Country, num_of_total_ports, i32);
    condition(Country, overextension_percentage, FixedPoint);
    condition(Country, owns, i32);
    condition(Country, owns_core_province, i32);
    condition(Country, prestige, FixedPoint);
    condition(Country, primary_culture, IdRef<Culture>);
    condition(Country, primitives, bool);
    condition(Country, reform_desire, FixedPoint);
    condition(Country, religion, IdRef<Religion>);
    condition(Country, religion_group, String);
    condition(Country, religion_years, ()); // XXX -- complex clauses...
    condition(Country, religious_school, ()); // XXX -- complex clauses...
    condition(Country, reverse_has_opinion, ()); // XXX -- complex clauses...
    condition(Country, ruler_age, i32);
    condition(Country, ruler_consort_marriage_length, i32);
    condition(Country, ruler_has_personality, String);
    condition(Country, ruler_religion, String);
    condition(Country, secondary_religion, IdRef<Religion>);
    condition(Country, stability, FixedPoint);
    condition(Country, TAG, IdRef<Country>); // XXX -- case sensitivity???
    condition(Country, tag, IdRef<Country>);
    condition(Country, tariff_value, FixedPoint);
    condition(Country, technology_group, String);
    condition(Country, trading_part, ()); // XXX -- complex clauses...
    condition(Country, treasury, FixedPoint);
    condition(Country, uses_blessings, bool);
    condition(Country, uses_karma, bool);
    condition(Country, vassal, u32);
    condition(Country, war_exhaustion, FixedPoint);
    condition(Country, war_score, FixedPoint);
    condition(Country, war_with, String);
    condition(Country, was_tag, IdRef<Country>);
    condition(Country, yearly_corruption_increase, FixedPoint);
    condition(Country, years_of_income, FixedPoint);

    // XXX: these are the <*> conditions
    condition(Province, enlightenment, FixedPoint);

    condition(Province, continent, IdRef<Continent>);
    condition(Province, development, u32);
    condition(Province, devastation, FixedPoint);
    condition(Province, has_building, String);
    condition(Province, has_climate, String);
    condition(Province, has_latent_trade_goods, IdRef<TradeGood>);
    condition(Province, has_province_flag, String);
    condition(Province, has_province_modifier, String);
    condition(Province, has_terrain, String);
    condition(Province, is_backing_current_issue, bool);
    condition(Province, is_blockaded, bool);
    condition(Province, is_capital, bool);
    condition(Province, is_city, bool);
    condition(Province, is_colony, bool);
    condition(Province, is_empty, bool);
    condition(Province, is_in_capital_area, bool);
    condition(Province, is_island, bool);
    condition(Province, is_looted, bool);
    condition(Province, is_node_in_trade_company_region, bool);
    condition(Province, is_overseas, bool);
    condition(Province, is_owned_by_trade_company, bool);
    condition(Province, is_prosperous, bool);
    condition(Province, is_reformation_center, bool);
    condition(Province, is_sea, bool);
    condition(Province, is_state, bool);
    condition(Province, is_territory, bool);
    condition(Province, is_wasteland, bool);
    condition(Province, province_id, i32);
    condition(Province, provincial_institution_progress, ()); // XXX -- complex
    condition(Province, trade_goods, IdRef<TradeGood>);

    // XXX: these are really *
    condition(Country, <scripted_trigger: ScriptedTrigger>, ()); // XXX -- arguments
    condition(Country, always, bool);
    condition(Country, current_age, String);
    condition(Country, custom_trigger_tooltip, CustomTrigger);
    condition(Country, has_global_flag, String);
    condition(Country, is_institution_enabled, String);
    condition(Country, is_month, u32);
    condition(Country, is_year, u32);
    condition(Country, num_of_electors, i32);
    condition(Country, num_of_foreign_hre_provinces, i32);
    condition(Country, papacy_active, bool);
    condition(Country, total_number_of_cardinals, i32);
}

pub type CountryCondition = Condition;
pub type ProvinceCondition = Condition;

impl paradox::Condition for Condition { }

#[derive(ParadoxParse, Default)]
pub struct Factor {
    pub factor: FixedPoint,
    #[modifiers] pub condition: Vec<CountryCondition>,
}

#[derive(ParadoxParse, Default)]
pub struct Weight {
    pub factor: FixedPoint,
    #[repeated] pub modifier: Vec<Factor>,
}

#[derive(ParadoxParse, Default)]
pub struct ScriptedTrigger {
    #[modifiers] pub conditions: Vec<Condition>
}

#[derive(ParadoxParse, Default, Debug)]
pub struct CustomTrigger {
    pub tooltip: String,
    #[modifiers] pub conditions: Vec<Condition>
}
