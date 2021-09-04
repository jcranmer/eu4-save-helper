use crate::{Eu4Atom, Eu4Trait};
use paradox::{FixedPoint, ParadoxParse, Parser, ParseError};

/*paradox::modifier_list! {
    modifier(Country, army_tradition, FixedPoint);
    modifier(Country, army_tradition_decay, FixedPoint);
    modifier(Country, army_tradition_from_battle, FixedPoint);
    modifier(Country, yearly_army_professionalism, FixedPoint);
    modifier(Country, drill_gain_modifier, FixedPoint);
    modifier(Country, drill_decay_modifier, FixedPoint);
    modifier(Country, infantry_cost, FixedPoint);
    modifier(Country, infantry_power, FixedPoint);
    modifier(Country, infantry_fire, FixedPoint);
    modifier(Country, infantry_shock, FixedPoint);
    modifier(Country, cavalry_cost, FixedPoint);
    modifier(Country, cavalry_power, FixedPoint);
    modifier(Country, cavalry_fire, FixedPoint);
    modifier(Country, cavalry_shock, FixedPoint);
    modifier(Country, artillery_cost, FixedPoint);
    modifier(Country, artillery_power, FixedPoint);
    modifier(Country, artillery_fire, FixedPoint);
    modifier(Country, artillery_shock, FixedPoint);
    modifier(Country, cav_to_inf_ratio, FixedPoint);
    modifier(Country, cavalry_flanking, FixedPoint);
    modifier(Country, artillery_bonus_vs_fort, i32);
    modifier(Country, backrow_artillery_damage, FixedPoint);
    modifier(Country, discipline, FixedPoint);
    modifier(Country, mercenary_discipline, FixedPoint);
    modifier(Country, land_morale, FixedPoint);
    modifier(Country, defensiveness, FixedPoint);
    modifier(Country, siege_ability, FixedPoint);
    modifier(Country, movement_speed, FixedPoint);
    modifier(Country, fire_damage, FixedPoint);
    modifier(Country, fire_damage_received, FixedPoint);
    modifier(Country, shock_damage, FixedPoint);
    modifier(Country, shock_damage_received, FixedPoint);
    modifier(Country, recover_army_morale_speed, FixedPoint);
    modifier(Country, siege_blockade_progress, i32);
    modifier(Country, reserves_organisation, FixedPoint);
    modifier(Country, land_attrition, FixedPoint);
    modifier(Country, reinforce_cost_modifier, FixedPoint);
    modifier(Country, reinforce_speed, FixedPoint);
    modifier(Country, manpower_recovery_speed, FixedPoint);
    modifier(Country, global_manpower, FixedPoint);
    modifier(Country, global_manpower_modifier, FixedPoint);
    modifier(Country, global_regiment_cost, FixedPoint);
    modifier(Country, global_regiment_recruit_speed, FixedPoint);
    modifier(Country, global_supply_limit_modifier, FixedPoint);
    modifier(Country, land_forcelimit, FixedPoint);
    modifier(Country, land_forcelimit_modifier, FixedPoint);
    modifier(Country, land_maintenance_modifier, FixedPoint);
    modifier(Country, mercenary_cost, FixedPoint);
    modifier(Country, merc_maintenance_modifier, FixedPoint);
    modifier(Country, possible_condottieri, FixedPoint);
    modifier(Country, possible_mercenaries, FixedPoint);
    modifier(Country, hostile_attrition, FixedPoint);
    modifier(Country, garrison_size, FixedPoint);
    modifier(Country, global_garrison_growth, FixedPoint);
    modifier(Country, fort_maintenance_modifier, FixedPoint);
    modifier(Country, rival_border_fort_maintenance, FixedPoint);
    modifier(Country, war_exhaustion, FixedPoint);
    modifier(Country, war_exhaustion_cost, FixedPoint);
    modifier(Country, leader_land_fire, u32);
    modifier(Country, leader_land_manuever, u32);
    modifier(Country, leader_land_shock, u32);
    modifier(Country, leader_siege, u32);
    modifier(Country, general_cost, FixedPoint);
    modifier(Country, free_leader_pool, i32);
    modifier(Country, raze_power_gain, FixedPoint);
    modifier(Country, loot_amount, FixedPoint);
    modifier(Country, available_province_loot, FixedPoint);
    modifier(Country, prestige_from_land, FixedPoint);
    modifier(Country, amount_of_banners, FixedPoint);
    modifier(Country, war_taxes_cost_modifier, FixedPoint);
    modifier(Country, leader_cost, FixedPoint);
    modifier(Country, special_unit_forcelimit, FixedPoint);
    modifier(Country, manpower_in_true_faith_provinces, FixedPoint);
    modifier(Country, mercenary_manpower, FixedPoint);
    modifier(Country, military_tactics, FixedPoint);
    modifier(Country, navy_tradition, FixedPoint);
    modifier(Country, navy_tradition_decay, FixedPoint);
    modifier(Country, naval_tradition_from_battle, FixedPoint);
    modifier(Country, naval_tradition_from_trade, FixedPoint);
    modifier(Country, heavy_ship_cost, FixedPoint);
    modifier(Country, heavy_ship_power, FixedPoint);
    modifier(Country, light_ship_cost, FixedPoint);
    modifier(Country, light_ship_power, FixedPoint);
    modifier(Country, galley_cost, FixedPoint);
    modifier(Country, galley_power, FixedPoint);
    modifier(Country, transport_cost, FixedPoint);
    modifier(Country, transport_power, FixedPoint);
    modifier(Country, global_ship_cost, FixedPoint);
    modifier(Country, global_ship_recruit_speed, FixedPoint);
    modifier(Country, global_ship_repair, FixedPoint);
    modifier(Country, naval_forcelimit, FixedPoint);
    modifier(Country, naval_forcelimit_modifier, FixedPoint);
    modifier(Country, naval_maintenance_modifier, FixedPoint);
    modifier(Country, global_sailors, FixedPoint);
    modifier(Country, global_sailors_modifier, FixedPoint);
    modifier(Country, sailor_maintenance_modifer, FixedPoint);
    modifier(Country, sailors_recovery_speed, FixedPoint);
    modifier(Country, blockade_efficiency, FixedPoint);
    modifier(Country, capture_ship_chance, FixedPoint);
    modifier(Country, global_naval_engagement_modifier, FixedPoint);
    modifier(Country, naval_attrition, FixedPoint);
    modifier(Country, naval_morale, FixedPoint);
    modifier(Country, ship_durability, FixedPoint);
    modifier(Country, sunk_ship_morale_hit_recieved, FixedPoint);
    modifier(Country, recover_navy_morale_speed, FixedPoint);
    modifier(Country, prestige_from_naval, FixedPoint);
    modifier(Country, leader_naval_fire, u32);
    modifier(Country, leader_naval_manuever, u32);
    modifier(Country, leader_naval_shock, u32);
    modifier(Country, own_coast_naval_combat_bonus, FixedPoint);
    modifier(Country, admiral_cost, FixedPoint);
    modifier(Country, admiral_skill_gain_modifier, FixedPoint);
    modifier(Country, movement_speed_onto_off_boat_modifier, FixedPoint);
    modifier(Country, global_naval_barrage_cost, FixedPoint);
    modifier(Country, allowed_marine_fraction, FixedPoint);
    modifier(Country, flagship_cost, FixedPoint);
    modifier(Country, disengagement_chance, FixedPoint);

    //modifier(Country, flagship_durability, type(1));
    //modifier(Country, flagship_morale, type(1));
    //modifier(Country, flagship_naval_engagement_modifier, type(1));
    //modifier(Country, flagship_landing_penalty, type(1));
    modifier(Country, number_of_cannons_flagship_modifier, FixedPoint);
    modifier(Country, naval_maintenance_flagship_modifier, FixedPoint);
    //modifier(Country, trade_power_in_fleet_modifier, type(1));
    modifier(Country, morale_in_fleet_modifier, FixedPoint);
    modifier(Country, blockade_impact_on_siege_in_fleet_modifier, FixedPoint);
    //modifier(Country, exploration_mission_range_in_fleet_modifier, type(100));
    modifier(Country, barrage_cost_in_fleet_modifier, FixedPoint);
    modifier(Country, naval_attrition_in_fleet_modifier, FixedPoint);
    modifier(Country, privateering_efficiency_in_fleet_modifier, FixedPoint);
    //modifier(Country, prestige_from_battles_in_fleet_modifier, type(1));
    //modifier(Country, naval_tradition_in_fleet_modifier, type(1));
    //modifier(Country, cannons_for_hunting_pirates_in_fleet, type(1));
    modifier(Country, movement_speed_in_fleet_modifier, FixedPoint);

    modifier(Country, diplomats, i32);
    modifier(Country, diplomatic_reputation, FixedPoint);
    modifier(Country, diplomatic_upkeep, i32);
    modifier(Country, envoy_travel_time, FixedPoint);
    modifier(Country, fabricate_claims_cost, FixedPoint);
    modifier(Country, improve_relation_modifier, FixedPoint);
    modifier(Country, vassal_forcelimit_bonus, FixedPoint);
    modifier(Country, vassal_income, FixedPoint);
    modifier(Country, ae_impact, FixedPoint);
    modifier(Country, claim_duration, FixedPoint);
    modifier(Country, diplomatic_annexation_cost, FixedPoint);
    modifier(Country, province_warscore_cost, FixedPoint);
    modifier(Country, unjustified_demands, FixedPoint);
    modifier(Country, enemy_core_creation, FixedPoint);
    modifier(Country, rival_change_cost, FixedPoint);
    modifier(Country, justify_trade_conflict_cost, FixedPoint);
    modifier(Country, stability_cost_to_declare_war, i32);
    modifier(Country, accept_vassalization_reasons, i32);

    modifier(Country, global_tax_income, FixedPoint);
    modifier(Country, global_tax_modifier, FixedPoint);
    modifier(Country, production_efficiency, FixedPoint);
    modifier(Country, state_maintenance_modifier, FixedPoint);
    modifier(Country, inflation_action_cost, FixedPoint);
    modifier(Country, inflation_reduction, FixedPoint);
    modifier(Country, interest, FixedPoint);
    modifier(Country, development_cost, FixedPoint);
    modifier(Country, tribal_development_growth, FixedPoint);
    modifier(Country, build_cost, FixedPoint);
    modifier(Country, build_time, FixedPoint);
    modifier(Country, great_project_upgrade_cost, FixedPoint);
    modifier(Country, administrative_efficiency, FixedPoint);
    modifier(Country, core_creation, FixedPoint);
    modifier(Country, core_decay_on_your_own, FixedPoint);

    modifier(Country, adm_tech_cost_modifier, FixedPoint);
    modifier(Country, dip_tech_cost_modifier, FixedPoint);
    modifier(Country, mil_tech_cost_modifier, FixedPoint);
    modifier(Country, technology_cost, FixedPoint);
    modifier(Country, idea_cost, FixedPoint);
    modifier(Country, embracement_cost, FixedPoint);
    modifier(Country, global_institution_spread, FixedPoint);
    modifier(Country, institution_spread_from_true_faith, FixedPoint);
    modifier(Country, native_advancement_cost, FixedPoint);
    modifier(Country, all_power_cost, FixedPoint);
    modifier(Country, innovativeness_gain, FixedPoint);
    modifier(Country, free_adm_policy, u32);
    modifier(Country, free_dip_policy, u32);
    modifier(Country, free_mil_policy, u32);
    modifier(Country, possible_adm_policy, u32);
    modifier(Country, possible_dip_policy, u32);
    modifier(Country, possible_mil_policy, u32);
    modifier(Country, possible_policy, u32);
    modifier(Country, free_policy, u32);

    modifier(Country, prestige, FixedPoint);
    modifier(Country, prestige_decay, FixedPoint);
    modifier(Country, monthly_splendor, FixedPoint);
    modifier(Country, yearly_corruption, FixedPoint);
    modifier(Country, advisor_cost, FixedPoint);
    modifier(Country, advisor_pool, u32);
    modifier(Country, female_advisor_chance, FixedPoint);
    modifier(Country, heir_chance, FixedPoint);
    modifier(Country, monthly_heir_claim_increase, FixedPoint);
    modifier(Country, block_introduce_heir, bool);
    modifier(Country, monarch_admin_power, u32);
    modifier(Country, monarch_diplomatic_power, u32);
    modifier(Country, monarch_military_power, u32);
    modifier(Country, adm_advisor_cost, FixedPoint);
    modifier(Country, dip_advisor_cost, FixedPoint);
    modifier(Country, mil_advisor_cost, FixedPoint);
    modifier(Country, monthly_support_heir_gain, FixedPoint);
    modifier(Country, power_projection_from_insults, FixedPoint);
    modifier(Country, monarch_lifespan, FixedPoint);

    modifier(Country, yearly_absolutism, FixedPoint);
    modifier(Country, max_absolutism, FixedPoint);
    modifier(Country, max_states, u32);
    modifier(Country, legitimacy, FixedPoint);
    modifier(Country, republican_tradition, FixedPoint);
    modifier(Country, devotion, FixedPoint);
    modifier(Country, horde_unity, FixedPoint);
    modifier(Country, meritocracy, FixedPoint);
    modifier(Country, monthly_militarized_society, FixedPoint);
    modifier(Country, yearly_tribal_allegiance, FixedPoint);
    modifier(Country, monthly_federation_favor_growth, FixedPoint);
    //modifier(Country, <faction>_influence, FixedPoint);
    modifier(Country, imperial_authority, FixedPoint);
    modifier(Country, imperial_authority_value, FixedPoint);
    modifier(Country, legitimate_subject_elector, FixedPoint);
    modifier(Country, imperial_mandate, FixedPoint);
    //modifier(Country, election_cycle, type(1));
    //modifier(Country, candidate_random_bonus, type(1));
    modifier(Country, reelection_cost, FixedPoint);
    modifier(Country, reform_progress_growth, FixedPoint);
    modifier(Country, monthly_reform_progress_modifier, FixedPoint);
    modifier(Country, governing_capacity, FixedPoint);
    modifier(Country, governing_capacity_modifier, FixedPoint);
    modifier(Country, governing_cost, FixedPoint);
    modifier(Country, state_governing_cost, FixedPoint);
    modifier(Country, trade_company_governing_cost, FixedPoint);
    modifier(Country, expand_administration_cost, FixedPoint);
    modifier(Country, yearly_revolutionary_zeal, FixedPoint);
    modifier(Country, max_revolutionary_zeal, FixedPoint);

    modifier(Country, culture_conversion_cost, FixedPoint);
    modifier(Country, num_accepted_cultures, i32);
    modifier(Country, same_culture_advisor_cost, FixedPoint);
    modifier(Country, promote_culture_cost, FixedPoint);

    modifier(Country, global_unrest, FixedPoint);
    modifier(Country, stability_cost_modifier, FixedPoint);
    modifier(Country, global_autonomy, FixedPoint);
    modifier(Country, min_autonomy, FixedPoint);
    modifier(Country, autonomy_change_time, FixedPoint);
    modifier(Country, harsh_treatment_cost, FixedPoint);
    modifier(Country, years_of_nationalism, i32);
    modifier(Country, min_autonomy_in_territories, FixedPoint);

    modifier(Country, liberty_desire, FixedPoint);
    modifier(Country, liberty_desire_from_subject_development, FixedPoint);
    modifier(Country, reduced_liberty_desire, FixedPoint);
    modifier(Country, reduced_liberty_desire_on_same_continent, FixedPoint);

    modifier(Country, spy_offence, FixedPoint);
    modifier(Country, global_spy_defence, FixedPoint);
    modifier(Country, discovered_relations_impact, FixedPoint);
    modifier(Country, rebel_support_efficiency, FixedPoint);

    modifier(Country, global_missionary_strength, FixedPoint);
    modifier(Country, global_heretic_missionary_strength, FixedPoint);
    modifier(Country, missionaries, u32);
    modifier(Country, missionary_maintenance_cost, FixedPoint);
    modifier(Country, religious_unity, FixedPoint);
    modifier(Country, tolerance_own, FixedPoint);
    modifier(Country, tolerance_heretic, FixedPoint);
    modifier(Country, tolerance_heathen, FixedPoint);
    modifier(Country, papal_influence, FixedPoint);
    modifier(Country, church_power_modifier, FixedPoint);
    modifier(Country, monthly_fervor_increase, FixedPoint);
    modifier(Country, harmonization_speed, FixedPoint);
    modifier(Country, yearly_harmony, FixedPoint);
    modifier(Country, monthly_piety, FixedPoint);
    modifier(Country, monthly_karma, FixedPoint);
    modifier(Country, enforce_religion_cost, FixedPoint);
    modifier(Country, prestige_per_development_from_conversion, FixedPoint);
    modifier(Country, warscore_cost_vs_other_religion, FixedPoint);
    modifier(Country, establish_order_cost, FixedPoint);
    modifier(Country, global_religious_conversion_resistance, FixedPoint);
    modifier(Country, curia_treasury_contribution, FixedPoint);
    modifier(Country, appoint_cardinal_cost, FixedPoint);

    modifier(Country, colonists, u32);
    modifier(Country, colonist_placement_chance, FixedPoint);
    modifier(Country, global_colonial_growth, i32);
    modifier(Country, range, FixedPoint);
    modifier(Country, native_uprising_chance, FixedPoint);
    modifier(Country, native_assimilation, FixedPoint);
    modifier(Country, migration_cooldown, FixedPoint);
    modifier(Country, migration_cost, FixedPoint);
    modifier(Country, global_tariffs, FixedPoint);
    modifier(Country, treasure_fleet_income, FixedPoint);
    modifier(Country, expel_minorities_cost, FixedPoint);

    modifier(Country, caravan_power, FixedPoint);
    modifier(Country, merchants, u32);
    modifier(Country, placed_merchant_power, FixedPoint);
    modifier(Country, global_trade_power, FixedPoint);
    modifier(Country, global_foreign_trade_power, FixedPoint);
    modifier(Country, global_own_trade_power, FixedPoint);
    modifier(Country, global_prov_trade_power_modifier, FixedPoint);
    modifier(Country, global_trade_goods_size_modifier, FixedPoint);
    modifier(Country, trade_efficiency, FixedPoint);
    modifier(Country, trade_range_modifier, FixedPoint);
    modifier(Country, trade_steering, FixedPoint);
    modifier(Country, global_ship_trade_power, FixedPoint);
    modifier(Country, privateer_efficiency, FixedPoint);
    modifier(Country, embargo_efficiency, FixedPoint);
    modifier(Country, ship_power_propagation, FixedPoint);
    modifier(Country, center_of_trade_upgrade_cost, FixedPoint);
    modifier(Country, trade_company_investment_cost, FixedPoint);
    modifier(Country, mercantilism_cost, FixedPoint);

    // XXX: this should be estate-based.
    modifier(Country, brahmins_hindu_loyalty_modifier, FixedPoint);
    modifier(Country, brahmins_muslim_loyalty_modifier, FixedPoint);
    modifier(Country, brahmins_other_loyalty_modifier, FixedPoint);
    modifier(Country, burghers_loyalty_modifier, FixedPoint);
    modifier(Country, church_loyalty_modifier, FixedPoint);
    modifier(Country, cossacks_loyalty_modifier, FixedPoint);
    modifier(Country, dhimmi_loyalty_modifier, FixedPoint);
    modifier(Country, jains_loyalty_modifier, FixedPoint);
    modifier(Country, maratha_loyalty_modifier, FixedPoint);
    modifier(Country, nobles_loyalty_modifier, FixedPoint);
    modifier(Country, nomadic_tribes_loyalty_modifier, FixedPoint);
    modifier(Country, rajput_loyalty_modifier, FixedPoint);
    modifier(Country, vaisyas_loyalty_modifier, FixedPoint);
    modifier(Country, brahmins_hindu_influence_modifier, FixedPoint);
    modifier(Country, burghers_influence_modifier, FixedPoint);
    modifier(Country, church_influence_modifier, FixedPoint);
    modifier(Country, nobles_influence_modifier, FixedPoint);
    modifier(Country, bureaucrats_influence, FixedPoint);
    modifier(Country, enuchs_influence, FixedPoint);
    modifier(Country, mr_aristocrats_influence, FixedPoint);
    modifier(Country, mr_guilds_influence, FixedPoint);
    modifier(Country, mr_traders_influence, FixedPoint);
    modifier(Country, pr_captains_influence, FixedPoint);
    modifier(Country, pr_smugglers_influence, FixedPoint);
    modifier(Country, rr_girondists_influence, FixedPoint);
    modifier(Country, rr_jacobins_influence, FixedPoint);
    modifier(Country, rr_royalists_influence, FixedPoint);
    modifier(Country, temples_influence, FixedPoint);

    /* == Province modifiers == */
    modifier(Province, institution_growth, i32);

    modifier(Province, max_attrition, i32);
    //modifier(Province, attrition, type(5));
    modifier(Province, local_hostile_attrition, FixedPoint);
    modifier(Province, fort_level, u32);
    modifier(Province, garrison_growth, FixedPoint);
    modifier(Province, local_defensiveness, FixedPoint);
    modifier(Province, local_friendly_movement_speed, FixedPoint);
    modifier(Province, local_hostile_movement_speed, FixedPoint);
    //modifier(Province, local_manpower, type(1));
    modifier(Province, local_manpower_modifier, FixedPoint);
    modifier(Province, local_regiment_cost, FixedPoint);
    modifier(Province, regiment_recruit_speed, FixedPoint);
    //modifier(Province, supply_limit, type(1));
    modifier(Province, supply_limit_modifier, FixedPoint);
    modifier(Province, local_amount_of_banners, FixedPoint);

    modifier(Province, local_naval_engagement_modifier, FixedPoint);
    modifier(Province, local_sailors, i32);
    modifier(Province, local_sailors_modifier, FixedPoint);
    modifier(Province, local_ship_cost, FixedPoint);
    modifier(Province, local_ship_repair, FixedPoint);
    modifier(Province, ship_recruit_speed, FixedPoint);

    modifier(Province, local_colonial_growth, i32);
    modifier(Province, local_colonist_placement_chance, FixedPoint);

    modifier(Province, inflation_reduction_local, FixedPoint);
    modifier(Province, local_state_maintenance_modifier, FixedPoint);
    modifier(Province, local_build_cost, FixedPoint);
    modifier(Province, local_build_time, FixedPoint);
    modifier(Province, local_monthly_devastation, FixedPoint);
    modifier(Province, local_production_efficiency, FixedPoint);
    modifier(Province, local_tax_modifier, FixedPoint);
    modifier(Province, tax_income, FixedPoint);
    modifier(Province, allowed_num_of_buildings, FixedPoint);
    modifier(Province, local_development_cost, FixedPoint);
    modifier(Province, local_institution_spread, FixedPoint);
    modifier(Province, local_core_creation, FixedPoint);
    modifier(Province, local_governing_cost, FixedPoint);

    modifier(Province, province_trade_power_modifier, FixedPoint);
    modifier(Province, province_trade_power_value, FixedPoint);
    modifier(Province, trade_goods_size_modifier, FixedPoint);
    modifier(Province, trade_goods_size, FixedPoint);
    modifier(Province, trade_value_modifier, FixedPoint);
    modifier(Province, trade_value, FixedPoint);

    modifier(Province, local_missionary_strength, FixedPoint);
    modifier(Province, local_religious_conversion_resistance, FixedPoint);
    modifier(Province, local_missionary_maintenance_cost, FixedPoint);
    modifier(Province, local_religious_unity_contribution, FixedPoint);

    modifier(Province, local_culture_conversion_cost, FixedPoint);

    modifier(Province, local_unrest, FixedPoint);
    modifier(Province, local_autonomy, FixedPoint);
    //modifier(Province, local_years_of_nationalism, type(-5));
    modifier(Province, min_local_autonomy, FixedPoint);

    /* Only usable in age bonuses */
    modifier(Country, attack_bonus_in_capital_terrain, bool);
    modifier(Country, can_bypass_forts, bool);
    modifier(Country, can_chain_claim, bool);
    modifier(Country, can_colony_boost_development, bool);
    modifier(Country, can_transfer_vassal_wargoal, bool);
    modifier(Country, force_march_free, bool);
    modifier(Country, free_maintenance_on_expl_conq, bool);
    modifier(Country, ignore_coring_distance, bool);

    /* Only usable in ideas */
    modifier(Country, cb_on_government_enemies, bool);
    modifier(Country, cb_on_religious_enemies, bool);
    modifier(Country, cb_on_primitives, bool);
    modifier(Country, cb_on_overseas, bool);
    modifier(Country, idea_claim_colonies, bool);
    modifier(Country, may_convert_territories, bool);
    modifier(Country, may_explore, bool);
    modifier(Country, may_perform_slave_raid, bool);
    modifier(Country, may_recruit_female_generals, bool);
    modifier(Country, no_religion_penalty, bool);
    modifier(Country, reduced_stab_impacts, bool);
    modifier(Country, sea_repair, bool);
    modifier(Country, may_establish_frontier, bool);
    modifier(Country, extra_manpower_at_religious_war, bool);
    modifier(Country, auto_explore_adjacent_to_colony, bool);
    modifier(Country, may_perform_slave_raid_on_same_religion, bool);
    modifier(Country, can_fabricate_for_vassals, bool);

    modifier(Country, has_banners, i32);
    modifier(Province, local_has_banners, i32);
    //modifier(Province, land_forcelimit, FixedPoint);
    //modifier(Province, naval_forcelimit, FixedPoint);

    // ???
    modifier(Province, allowed_num_of_manufactories, i32);
    modifier(Province, local_governing_cost_increase, i32);
    modifier(Province, state_governing_cost_increase, i32);
}*/

#[derive(ParadoxParse, Default)]
pub struct EventModifier {
    #[optional] pub picture: String,
    #[optional] pub expire_message_type: String,
    #[optional] pub religion: bool,
    #[optional] pub secondary_religion: bool,
    #[optional] pub religion_sub_modifier: bool,
    #[optional] pub is_scholar_modifier: bool,
    #[optional] pub is_janissary_modifier: bool,
    #[optional] pub is_mercenary_modifier: bool,
    #[optional] pub is_revolutionary_guard_modifier: bool,
    #[optional] pub is_rajput_modifier: bool,
    #[modifiers] pub modifiers: Modifiers,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ModifierValue {
    Bool(bool),
    Integer(i32),
    FixedPoint(FixedPoint)
}

impl ModifierValue {
    pub fn as_fixed_point(self) -> FixedPoint {
        match self {
            Self::Bool(true) => FixedPoint::ONE,
            Self::Bool(false) => FixedPoint::ZERO,
            Self::Integer(i) => i.into(),
            Self::FixedPoint(f) => f
        }
    }
}

impl Default for ModifierValue {
    fn default() -> Self {
        Self::Bool(false)
    }
}

impl core::ops::Add for ModifierValue {
    type Output = ModifierValue;
    fn add(self, other: ModifierValue) -> ModifierValue {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => Self::Bool(a | b),
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a + b),
            (Self::FixedPoint(a), Self::FixedPoint(b)) =>
                Self::FixedPoint(a + b),
            (Self::Integer(a), Self::FixedPoint(b)) =>
                Self::FixedPoint(FixedPoint::from(a) + b),
            (Self::FixedPoint(a), Self::Integer(b)) =>
                Self::FixedPoint(a + FixedPoint::from(b)),
            (Self::Bool(false), t) => t,
            (a, b) => panic!("Cannot combine {:?} with {:?}", a, b)
        }
    }
}

impl core::ops::Mul<FixedPoint> for ModifierValue {
    type Output = ModifierValue;
    fn mul(self, scale: FixedPoint) -> ModifierValue {
        match self {
            Self::Bool(a) => Self::Bool(a),
            Self::Integer(a) => Self::FixedPoint(FixedPoint::from(a) * scale),
            Self::FixedPoint(a) => Self::FixedPoint(a * scale)
        }
    }
}

#[derive(Default, Debug)]
pub struct Modifiers {
    pub modifiers: Vec<(Eu4Atom, ModifierValue)>,
}

impl Modifiers {
    pub fn read_field(&mut self, key: Eu4Atom,
                      parser: &mut Parser<Eu4Trait>) -> Result<(), ParseError> {
        let value = parser.get_token()?.ok_or(ParseError::Eof)?;
        let str_value = value.try_to_string()?;
        let value = if str_value == "yes" {
            ModifierValue::Bool(true)
        } else if str_value == "no" {
            ModifierValue::Bool(false)
        } else {
            ModifierValue::FixedPoint(
                str_value.parse()
                    .map_err(|e| ParseError::Conversion(Box::new(e)))?
            )
        };
        self[&key] = value;
        Ok(())
    }

    pub fn add_modifiers(&mut self, other: &Modifiers) -> &mut Self {
        for (key, value) in &other.modifiers {
            let our_mod = &mut self[key];
            *our_mod = *our_mod + *value;
        }
        self
    }

    pub fn add_scaled_modifiers(&mut self, other: &Modifiers,
                                scale: FixedPoint) -> &mut Self {
        for (key, value) in &other.modifiers {
            let our_mod = &mut self[key];
            *our_mod = *our_mod + (*value * scale);
        }
        self
    }
}

impl core::ops::Index<&'_ Eu4Atom> for Modifiers {
    type Output = ModifierValue;
    fn index(&self, idx: &Eu4Atom) -> &ModifierValue {
        static DUMMY_VAL : ModifierValue = ModifierValue::Bool(false);
        self.modifiers.binary_search_by_key(idx, |(k, _)| k.clone())
            .map(|idx| &self.modifiers[idx].1)
            .unwrap_or(&DUMMY_VAL)
    }
}

impl core::ops::IndexMut<&'_ Eu4Atom> for Modifiers {
    fn index_mut(&mut self, key: &Eu4Atom) -> &mut ModifierValue {
        match self.modifiers.binary_search_by_key(key, |(k, _)| k.clone()) {
            Ok(idx) => &mut self.modifiers[idx].1,
            Err(idx) => {
                self.modifiers.insert(idx, (key.clone(), Default::default()));
                &mut self.modifiers[idx].1
            }
        }
    }
}

impl ParadoxParse<Eu4Trait> for Modifiers {
    fn read(&mut self, parser: &mut Parser<Eu4Trait>) -> Result<(), ParseError> {
        parser.parse_key_scope(|key, parser| {
            self.read_field(key, parser)
        })
    }
}
