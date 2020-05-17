use paradox::FixedPoint;
use crate::{GameData, Gamestate};

paradox::effect_list! {
    // This is h4x0rs, but it'll do for now...
    fn r#if(cheat: (), scope: Country) {}
    fn every_known_country(cheat: (), scope: Country) {}
//    fn set_global_flag(flag: String) {
  //      state.flags[flag] = state.date;
    //}
/*
    fn set_global_flag(<code><flag></code><br>A unique string to identify the global flag with.) {}
    fn clr_global_flag(<code><flag></code><br>The unique string of a global flag to clear.) {}
    fn custom_tooltip(<code><string></code><br>An localized string to display in the tooltip) {}
    fn log(<code><string></code><br>An string to in the game.log) {}
    fn save_event_target_as(<code><string></code><br>A unique string to identify the event target with.) {}
    fn save_global_event_target_as(<code><string></code><br>A unique string to identify the global event target with.) {}
    fn clear_global_event_target(<code><string></code><br>The unique string of the global event target to clear.) {}
    fn clear_global_event_targets(<code>yes</code><br>Boolean.) {}
    fn show_ambient_object(<code><string></code><br>The unique string of the ambient object to show.) {}
    fn hide_ambient_object(<code><string></code><br>The unique string of the ambient object to hide.) {}
*/

    //fn <unit name>(<code><scope></code>, scope: Country) {}
    fn add_absolutism(val: i32, scope: Country) {}
    fn add_accepted_culture(culture: String, scope: Country) {}
    //fn add_active_policy(policy: (), scope: Country) {}
    fn add_adm_power(val: i32, scope: Country) {}
    fn add_adm_tech(val: i32, scope: Country) {}
    //fn add_army_professionalism(float: (), scope: Country) {}
    fn add_army_tradition(val: i32, scope: Country) {}
    fn add_authority(val: i32, scope: Country) {}
    //fn add_casus_belli(target: String, type: String, months: i32, scope: Country) {}
    //fn add_church_aspect(aspect: (), scope: Country) {}
    fn add_church_power(val: i32, scope: Country) {}
    //fn add_corruption(float: (), scope: Country) {}*/
    fn add_country_modifier(name: String, duration: i32, /* hidden: bool, desc: String */ scope: Country) {}
    fn add_devotion(val: i32, scope: Country) {}
    fn add_dip_power(val: i32, scope: Country) {}
    fn add_dip_tech(val: i32, scope: Country) {}
    /*fn add_disaster_modifier(name: modifier, The event modifier to add., disaster: disaster, The disaster to apply the modifier to., duration: days, The duration for the modifier to be active for.) {}
    fn add_disaster_progress(disaster: disaster, The disaster to alter., value: int, The amount of progress to add or subtract.) {}
    fn add_doom(val: i32, scope: Country) {}
    fn add_estate_influence_modifier(estate: estate, The estate to apply the modifier to., desc: string, The text to display in the Influence tooltip for this modifier., influence: int, The amount of influence to add or subtract., duration: days, The duration for which this modifier is active.<br>) {}
    fn add_estate_loyalty(estate: estate, The estate to apply the loyalty to., loyalty: int, The amount of loyalty to add or subtract.<br>) {}
    fn add_estate_loyalty_modifier(estate: estate, The estate to apply the modifier to., desc: string, The text to display in the Influence tooltip for this modifier., loyalty: int, The amount of loyalty to add or subtract., duration: days, The duration for which this modifier is active.<br>) {}
    fn add_faction(faction: (), scope: Country) {}
    fn add_faction_influence(faction: faction, The faction to add influence to., influence: int, The amount of influence to add.) {}
    fn add_fervor(val: i32, scope: Country) {}
    fn add_government_power(government_mechanic: type, Which government mechanic to add power to., which: type, Which pool type to add the power to. ''ADM, DIP or MIL'', amount: type, The amount of power to add.) {}
    fn add_government_reform(reform: (), scope: Country) {}
    fn add_harmonization_progress(val: i32, scope: Country) {}
    fn add_harmonized_religion(religion: (), scope: Country) {}
    fn add_harmony(val: i32, scope: Country) {}
    fn add_heir_claim(val: i32, scope: Country) {}
    fn add_heir_personality(personality: (), scope: Country) {}
    fn add_heir_support(val: i32, scope: Country) {}
    fn add_historical_friend(scope: (), scope: Country) {}
    fn add_historical_rival(scope: (), scope: Country) {}
    fn add_horde_unity(<code><int></code>, The amount of horde unity to add or subtract.<br>) {}
    fn add_idea(idea: (), scope: Country) {}
    fn add_idea_group(ideagroup: (), scope: Country) {}
    fn add_imperial_influence(val: i32, scope: Country) {}
    fn add_incident_variable_value(incident: incident, The incident to change., value: int, The amount to add to the incident variable.) {}
    fn add_inflation(float: (), scope: Country) {}
    fn add_isolationism(val: i32, scope: Country) {}
    fn add_karma(val: i32, scope: Country) {}
    fn add_legitimacy(<code><int></code>, The amount of legitimacy to add or subtract., <code><scope></code>, The amount of legitimacy to add or subtract, taking the value from the scope's current value.<br>) {}
    fn add_liberty_desire(val: i32, scope: Country) {}
    fn add_loan(interest_modifier: i32, fixed_interest: bool, duration: i32, scope: Country) {}
    fn add_mandate(val: i32, scope: Country) {}
    fn add_manpower(float: (), scope: Country) {}
    fn add_mercantilism(val: i32, scope: Country) {}
    fn add_meritocracy(<code><int></code>, The amount of meritocracy to add or subtract.<br>) {}
    fn add_mil_power(val: i32, scope: Country) {}
    fn add_mil_tech(val: i32, scope: Country) {}
    fn add_militarised_society(<code><int></code>, The amount of militarisation to add or subtract.<br>) {}
    fn add_navy_tradition(val: i32, scope: Country) {}
    fn add_next_institution_embracement(val: i32, scope: Country) {}
    fn add_opinion(who: scope, The country the opinion modifier is for., modifier: modifier, The opinion modifier to use., years: int, Optional. Whether the modifier expires after a set amount of years.) {}
    fn add_papal_influence(val: i32, scope: Country) {}
    fn add_patriarch_authority(float: (), scope: Country) {}
    fn add_piety(float: (), scope: Country) {}
    */
    fn add_prestige(value: FixedPoint, scope: Country) {
        scope.prestige += value;
    }
    /*
    fn add_queen_personality(personality: (), scope: Country) {}
    fn add_reform_desire(float: (), scope: Country) {}
    fn add_republican_tradition(<code><int></code>, The amount of republican tradition to add or subtract.<br>) {}
    fn add_ruler_modifier(name: name, The event modifier to add., duration: days, Optional. The number of days to add the ruler modifier for., <code>hidden = yes</code>, Optional. Whether the ruler modifier is hidden in the government view., <code>desc = <string</code>, Optional. The string used to override the automatic duration string.) {}
    fn add_ruler_personality(personality: (), scope: Country) {}
    fn add_sailors(val: i32, scope: Country) {}
    fn add_scaled_imperial_influence(val: i32, scope: Country) {}
    fn add_scaled_republican_tradition(<code><int></code>, The amount of republican tradition to add or subtract.<br>) {}
    fn add_splendor(val: i32, scope: Country) {}
    fn add_spy_network_from(who: scope, The country to add spy network for., value: int, The amount of spy network to add or subtract.) {}
    fn add_spy_network_in(who: scope, The country to add spy network in., value: int, The amount of spy network to add or subtract.) {}*/
    fn add_stability(val: i32, scope: Country) {}
    /*fn add_tariff_value(float: (), scope: Country) {}
    fn add_treasury(val: i32, scope: Country) {}
    fn add_tribal_allegiance(<code><int></code>, The amount of tribal allegiance to add or subtract.<br>) {}
    fn add_truce_with(scope: (), scope: Country) {}
    fn add_trust(who: scope, The country trust will be added with., value: int, The amount of trust to add or subtract., <code>mutual = yes</code>, Optional. Whether to apply the trust change to both countries.) {}
    fn add_war_exhaustion(val: i32, scope: Country) {}
    fn add_yearly_manpower(float: (), scope: Country) {}
    fn add_yearly_sailors(float: (), scope: Country) {}
    fn add_years_of_income(float: (), scope: Country) {}
    fn adopt_reform_progress(<code><country scope></code>, scope: Country) {}
    fn artillery(scope: (), scope: Country) {}
    fn break_marriage(scope: (), scope: Country) {}
    fn break_union(scope: (), scope: Country) {}
    fn cavalry(scope: (), scope: Country) {}
    fn change_adm(val: i32, scope: Country) {}
    fn change_consort_regent_to_ruler(<code>yes</code>, Boolean.) {}
    fn change_dip(val: i32, scope: Country) {}
    fn change_government(government: (), scope: Country) {}
    fn change_government_reform_progress(val: i32, scope: Country) {}
    fn change_graphical_culture(gfxculture: (), scope: Country) {}
    fn change_heir_adm(val: i32, scope: Country) {}
    fn change_heir_dip(val: i32, scope: Country) {}
    fn change_heir_mil(val: i32, scope: Country) {}
    fn change_innovativeness(val: i32, scope: Country) {}
    fn change_mil(val: i32, scope: Country) {}
    fn change_personal_deity(deity: (), scope: Country) {}
    fn change_price(trade_goods: tradegood, The trade good to change., key: string, The localisation key to display in the Trade Good's price tooltip., value: float, The percentage to change the value by., duration: days, The duration for the value to stay changed for.) {}
    fn change_primary_culture(<code><culture></code>, The culture to change to, from {{path|common/cultures/}}, <code><scope></code>, The culture to change to, derived from the scope.) {}
    */
    fn change_religion(religion: String, scope: Country) {}
    /*
    fn change_statists_vs_orangists(<code><int></code>, The amount of swing towards statists or organgists.<br>) {}
    fn change_tag(scope: (), scope: Country) {}
    fn change_technology_group(<code><technology group></code>, The technology group to change to.) {}
    fn change_unit_type(type: (), scope: Country) {}
    fn clear_saved_name(key: (), scope: Country) {}
    fn clr_consort_flag(flag: (), scope: Country) {}
    fn clr_country_flag(<code><flag></code><br>The unique string of a country flag to clear.) {}
    fn clr_heir_flag(flag: (), scope: Country) {}
    fn clr_ruler_flag(flag: (), scope: Country) {}
    fn collapse_nation(<code>yes</code>, scope: Country) {}
    fn complete_mission(<code><key></code>, scope: Country) {}
    fn convert_female_heir_to_general(<code>yes</code>, Boolean.) {}
    fn convert_female_ruler_to_general(<code>yes</code>, Boolean.) {}
    fn convert_heir_to_general(<code>yes</code>, Boolean.) {}
    fn convert_ruler_to_general(<code>yes</code>, Boolean.) {}
    fn country_event(id: id, days: days, random: days, tooltip: string, scope: Country) {}
    fn create_admiral(tradition: int,add_fire: int,add_shock: int,add_manuever: int,add_siege: int,culture: identifier, scope: Country) {}
    fn create_advisor(advisor: (), scope: Country) {}
    fn create_alliance(scope: (), scope: Country) {}
    fn create_conquistador(val: i32, scope: Country) {}
    fn create_explorer(val: i32, scope: Country) {}
    fn create_general(tradition: int,add_fire: int,add_shock: int,add_manuever: int,add_siege: int,culture: identifier, scope: Country) {}
    fn create_guarantee(scope: (), scope: Country) {}
    fn create_independent_estate(estate: (), scope: Country) {}
    fn create_independent_estate_from_religion(estate: (), scope: Country) {}
    fn create_march(scope: (), scope: Country) {}
    fn create_marriage(scope: (), scope: Country) {}
    fn create_subject(subject_type: type, The subject type to use., subject: scope, The scope to make the subject.) {}
    fn create_union(scope: (), scope: Country) {}
    fn create_vassal(scope: (), scope: Country) {}
    fn declare_war(scope: (), scope: Country) {}
    fn declare_war_with_cb(who: scope, The country declared against., casus_belli: cb, The casus belli to use., war_goal_province: province id, Optional. Defines the province the war was declared for, if required by the casus belli.<br>) {}
    fn define_admiral(shock: int, The shock pips of the leader., fire: int, The fire pips of the leader., manuever: int, The maneuver pips of the leader., siege: int, The siege pips of the leader., name: string, Optional. The name of the leader., <code>female = yes</code>, Optional. Makes the leader female., trait: trait, Optional. A trait automatically assigned to the leader.) {}
    fn define_advisor(type: type, The advisor type to create., skill: int, The amount of skill the advisor has. Limited to 3 maximum., name: string, Optional. The name to use., location: province id, Optional. The province that the advisor comes from., <code>discount = yes</code>, Optional. If the SCRIPTED_ADVISOR_DISCOUNT cost reduction applies to this advisor., <code>female = yes</code>, Optional. Makes this advisor female., <code>culture = <culture> / <scope></code>, Optional. The culture of this advisor., <code>religion = <religion> / <scope></code>, Optional. The religion of this advisor.) {}
    fn define_conquistador(shock: int, The shock pips of the leader., fire: int, The fire pips of the leader., manuever: int, The maneuver pips of the leader., siege: int, The siege pips of the leader., name: string, Optional. The name of the leader., <code>female = yes</code>, Optional. Makes the leader female., trait: trait, Optional. A trait automatically assigned to the leader.) {}
    fn define_consort(name: string, Optional. The name of the consort., country_of_origin: scope, Optional. The origin country of the consort., <code>dynasty = <string> / <scope> / original_dynasty</code>, Optional. The dynasty of the consort., age: years, Optional. The age of the consort., adm: int, Optional. The ADM skill for the consort., dip: int, Optional. The DIP skill for the consort., mil: int, Optional. The MIL skill for the consort., <code>hide_skills = yes</code>, Optional. Hides the skill values for this consort., <code>female = yes</code>, Optional. Forces the consort to be female., <code>male = yes</code>, Optional. Forces the consort to be male., <code>culture = <culture> / <scope></code>, Optional. The culture of the consort., <code>religion = <religion> / <scope></code>, Optional. The religion of the consort.<br>) {}
    fn define_explorer(shock: int, The shock pips of the leader., fire: int, The fire pips of the leader., manuever: int, The maneuver pips of the leader., siege: int, The siege pips of the leader., name: string, Optional. The name of the leader., <code>female = yes</code>, Optional. Makes the leader female., trait: trait, Optional. A trait automatically assigned to the leader.) {}
    fn define_general(shock: int, The shock pips of the leader., fire: int, The fire pips of the leader., manuever: int, The maneuver pips of the leader., siege: int, The siege pips of the leader., name: string, Optional. The name of the leader., <code>female = yes</code>, Optional. Makes the leader female., trait: trait, Optional. A trait automatically assigned to the leader.) {}
    fn define_heir(name: string, Optional. The name of the heir., <code>dynasty = <string> / <scope> / original_dynasty</code>, Optional. The dynasty of the heir., age: years, Optional. The age of the heir., birth_date: date, Optional. The birth date of the heir. Can be used instead of the ''age'' parameter., claim: int, Optional. The starting legitimacy of the heir., adm: int, Optional. The minimum ADM skill roll for the heir., dip: int, Optional. The minimum DIP skill roll for the heir., mil: int, Optional. The minimum MIL skill roll for the heir., <code>fixed = yes</code>, Optional. Makes the heir skill roll fixed to the defined minimums., max_random_adm: int, Optional. The maximum ADM skill roll for the heir., max_random_dip: int, Optional. The maximum DIP skill roll for the heir., max_random_mil: int, Optional. The maximum MIL skill roll for the heir., <code>hide_skills = yes</code>, Optional. Hides the skill values for this heir.,<code>hidden = yes</code>, Optional. Hides the skill values for this heir. Legacy version of ''hide_skills''.,<code>no_consort_with_heir = yes</code>, Optional. Makes the heir a child not of the consort, i.e. a bastard,<code>female = yes</code>, Optional. Forces the heir to be female., <code>male = yes</code>, Optional. Forces the heir to be male., <code>culture = <culture> / <scope></code>, Optional. The culture of the heir., <code>religion = <religion> / <scope></code>, Optional. The religion of the heir.<br>) {}
    fn define_heir_to_general(shock: int, The shock pips of the leader., fire: int, The fire pips of the leader., manuever: int, The maneuver pips of the leader., siege: int, The siege pips of the leader.<br>) {}
    fn define_leader_to_ruler(<code>type = general / conquistador / admiral / explorer</code>, The leader type to pick a leader from., name: string, The specific leader to use. Used instead of the ''type'' parameter., claim: int, Optional. The starting legitimacy of the ruler., adm: int, Optional. The minimum ADM skill roll for the ruler., dip: int, Optional. The minimum DIP skill roll for the ruler., mil: int, Optional. The minimum MIL skill roll for the ruler., <code>fixed = yes</code>, Optional. Makes the ruler skill roll fixed to the defined minimums.<br>) {}
    fn define_ruler(name: string, dynasty: String, age: i32, claim: FixedPoint, adm: i32, dip: i32, mil: i32, fixed : bool, max_random_adm: i32, max_random_dip: i32, max_random_mil: i32, hide_skills: bool, scope: Country) {}
    fn define_ruler_to_general(shock: int, The shock pips of the leader., fire: int, The fire pips of the leader., manuever: int, The maneuver pips of the leader., siege: int, The siege pips of the leader.<br>) {}
    fn disband_rebels(type: (), scope: Country) {}
    fn dismantle_empire_of_china(<code><yes> / <no></code>, Boolean.) {}
    fn dismantle_hre(<code>yes</code>, Boolean.) {}
    fn dissolve_parliament(<code>yes</code>, Boolean.) {}
    fn elector(<code><scope></code>, The country to make elector., <code>yes / no</code>, Boolean.) {}
    fn enable_hre_leagues(<code>yes</code>, Boolean.) {}
    fn enable_religion(religion: (), scope: Country) {}
    fn end_disaster(disaster: (), scope: Country) {}
    fn excommunicate(scope: (), scope: Country) {}
    fn exile_heir_as(string: (), scope: Country) {}
    fn exile_ruler_as(string: (), scope: Country) {}
    fn force_converted(<code>yes</code>, Boolean.) {}
    fn form_coalition_against(scope: (), scope: Country) {}
    fn galley(scope: (), scope: Country) {}
    fn grant_independence(<code>yes</code>, Boolean.) {}
    fn heavy_ship(scope: (), scope: Country) {}
    fn hre_inheritable(<code>yes</code>, Boolean.) {}
    fn imperial_ban_allowed(<code>yes</code>, Boolean.) {}
    fn infantry(scope: (), scope: Country) {}
    fn inherit(scope: (), scope: Country) {}
    fn internal_hre_cb(<code>no</code>, Boolean.) {}
    fn join_league(scope: (), scope: Country) {}
    fn kill_advisor(<code><random></code>, Kill a random advisor., <code><advisor></code>, Kill a specific advisor type., <code><advisor id></code>, Kill a specific advisor id.<br>) {}
    fn kill_heir(<code>yes</code>, Boolean.) {}
    fn kill_leader(type: type, The leader type to kill. ''name'' is the specific leader name to target.) {}
    fn kill_ruler(<code>yes</code>, Boolean.) {}
    fn leave_league(scope: (), scope: Country) {}
    fn light_ship(scope: (), scope: Country) {}
    fn loan_size(val: i32, scope: Country) {}
    fn mercenary_cavalry(scope: (), scope: Country) {}
    fn mercenary_infantry(scope: (), scope: Country) {}
    fn reinstate_parliament(<code>yes</code>, Boolean.) {}
    fn release(scope: (), scope: Country) {}
    fn release_all_subjects(<code>yes</code>, Boolean.) {}
    fn remove_accepted_culture(<code><culture></code>, The culture to remove, from {{path|common/cultures/}}, <code><scope></code>, The culture to remove, derived from the scope.) {}
    fn remove_advisor(<code><random></code>, Remove a random advisor., <code><advisor></code>, Remove a specific advisor type, <code><advisor id></code>, Remove a specific advisor id.<br>) {}
    fn remove_advisor_by_category(type: (), scope: Country) {}
    fn remove_casus_belli(target: scope, The country the casus belli is against., type: cb, The casus belli to remove.<br>) {}
    fn remove_church_aspect(aspect: (), scope: Country) {}
    fn remove_consort(<code>yes</code>, Boolean.) {}*/
    fn remove_country_modifier(modifier: String, scope: Country) {}
    /*fn remove_faction(faction: (), scope: Country) {}
    fn remove_fow(months: (), scope: Country) {}
    fn remove_heir(<code>yes</code>, Boolean.) {}
    fn remove_heir_personality(personality: (), scope: Country) {}
    fn remove_historical_friend(scope: (), scope: Country) {}
    fn remove_historical_rival(scope: (), scope: Country) {}
    fn remove_idea(idea: (), scope: Country) {}
    fn remove_idea_group(ideagroup: (), scope: Country) {}
    fn remove_opinion(who: scope, The country the opinion modifier is held against., modifier: modifier, The opinion modifier to use.<br>) {}
    fn remove_queen_personality(personality: (), scope: Country) {}
    fn remove_religious_reforms(val: i32, scope: Country) {}
    fn remove_ruler_personality(personality: (), scope: Country) {}
    fn reverse_add_casus_belli(target: scope, The country to that gains the casus belli., type: cb, The casus belli to add., months: months, The number of months the casus belli lasts for.) {}
    fn reverse_add_opinion(who: scope, The country the opinion modifier is added to., modifier: modifier, The opinion modifier to use., years: int, Optional. Whether the modifier expires after a set amount of years.) {}
    fn reverse_remove_casus_belli(target: scope, The country that holds the casus belli., type: cb, The casus belli to remove.<br>) {}
    fn reverse_remove_opinion(who: scope, The country the opinion modifier holds., modifier: modifier, The opinion modifier to use.<br>) {}
    fn revoke_reform(reform: (), scope: Country) {}
    fn set_ai_attitude(attitude: type, The attitude to use., who: scope, Who the attitude is directed against., <code>locked = <yes> / <no></code>, Whether the attitude can be re-evaluated and changed.) {}
    fn set_ai_personality(personality: type, The personality to use., <code>locked = <yes> / <no></code>, Whether the personality can be re-evaluated and changed.) {}
    fn set_allow_female_emperor(<code>yes</code>, Boolean.) {}
    fn set_consort_culture(<code><culture></code>, The culture to change to.<br>) {}
    fn set_consort_flag(flag: (), scope: Country) {}
    fn set_consort_religion(<code><religion></code>, The religion to change to.<br>) {}*/
    fn set_country_flag(flag: String, scope: Country) {}
    /*fn set_dynasty(<code><string> / <scope> / original_dynasty</code>, The dynasty to set.<br>) {}
    fn set_emperor_of_china(scope: (), scope: Country) {}
    fn set_government_and_rank(government: government, The government to change to., rank: int, The rank to change to.) {}
    fn set_government_rank(val: i32, scope: Country) {}
    fn set_heir(string: (), scope: Country) {}
    fn set_heir_culture(<code><culture></code>, The culture to change to.<br>) {}
    fn set_heir_flag(flag: (), scope: Country) {}
    fn set_heir_religion(<code><religion></code>, The religion to change to.<br>) {}
    fn set_hre_heretic_religion(religion: (), scope: Country) {}
    fn set_hre_religion(religion: (), scope: Country) {}
    fn set_hre_religion_locked(<code>yes</code>, Boolean.) {}
    fn set_hre_religion_treaty(<code>yes</code>, Boolean.) {}
    fn set_in_empire(<code>yes</code>, Boolean.) {}
    fn set_incident_variable_value(incident: incident, The incident to change., value: int, The amount to set the incident variable to.) {}
    fn set_isolationism(val: i32, scope: Country) {}
    fn set_karma(val: i32, scope: Country) {}
    fn set_legacy_government(government: government, The pre-Dharma government type to change to.) {}
    fn set_mandate(val: i32, scope: Country) {}
    fn set_meritocracy(<code><int></code>, The amount of meritocracy to set.<br>) {}
    fn set_papacy_active(<code>yes</code><code>no</code>, Boolean.) {}
    fn set_primitive(<code><yes><no></code>, Boolean.) {}
    fn set_revolution_target(scope: (), scope: Country) {}
    fn set_ruler(string: (), scope: Country) {}
    fn set_ruler_culture(<code><culture></code>, The culture to change to.<br>) {}
    fn set_ruler_flag(flag: (), scope: Country) {}
    fn set_ruler_religion(<code><religion></code>, The religion to change to.<br>) {}
    fn set_saved_name(key: string, The key that holds the name., <code>type = advisor / simple</code>, The name generation type. Advisor draws from namelist, simple utilises name parameter., name: string, The name to hold in the key. Only used with the ''simple'' type., scope: scope, The country scope to draw the namelist from. Only used with the ''advisor'' type.<br>) {}
    fn set_school_opinion(who: who, Which country to alter opinion with., opinion: opinion, Which opinion state to change to.<br>) {}
    fn swap_free_idea_group(yes: (), scope: Country) {}
    fn switch_tag(scope: (), scope: Country) {}
    fn transport(scope: (), scope: Country) {}
    fn unlock_cult(cult: (), scope: Country) {}
    fn vassalize(scope: (), scope: Country) {}
    fn white_peace(scope: (), scope: Country) {}

/////// PROVINCE SCOPE //////
|- @@@@@ <rebel type> @@@@@ <code><int></code>, The size to use. @@@@@ <code>anti_tax_rebels = 1</code>
|- @@@@@ <unit name> @@@@@ <code><scope></code>, The country scope the unit belongs to @@@@@ <pre>capital_scope = {
    fn add_base_manpower(val: i32, scope: Province) {}
    fn add_base_production(val: i32, scope: Province) {}
    fn add_base_tax(val: i32, scope: Province) {}
    fn add_building(building: (), scope: Province) {}
    fn add_building_construction(building: building, The building to construct., speed: float, The speed percentage of the base building speed to use., cost: float, The cost percentage of the base building cost to use.<br>) {}
    fn add_cardinal(<code>yes</code>, Boolean.) {}
    fn add_center_of_trade_level(<code><int></code>, Amount to increase center of trade level.<br>) {}
    fn add_claim(scope: (), scope: Province) {}
    fn add_colonysize(val: i32, scope: Province) {}
    fn add_construction_progress(float: (), scope: Province) {}
    fn add_core(scope: (), scope: Province) {}
    fn add_core_construction(<code>yes</code><br> Boolean., speed: float, The speed percentage of the base coring speed to use., cost: float, The cost percentage of the base coring cost to use.) {}
    fn add_culture_construction(<code>yes</code>, Boolean., speed: float, The speed percentage of the base conversion speed to use., cost: float, The cost percentage of the base conversion cost to use.<br>) {}
    fn add_devastation(val: i32, scope: Province) {}
    fn add_great_project(project: (), scope: Province) {}
    fn add_institution_embracement(which: institution, The institution to add to., value: int, The amount of embracement to add or subtract.) {}
    fn add_local_autonomy(val: i32, scope: Province) {}
    fn add_nationalism(val: i32, scope: Province) {}
    fn add_permanent_claim(scope: (), scope: Province) {}
    fn add_permanent_province_modifier(name: name, The event modifier to add., duration: days, Optional. The number of days to add the province modifier for., <code>hidden = yes</code>, Optional. Whether the province modifier is hidden in the province view., desc: string, Optional. The string used to override the automatic duration string.) {}
    fn add_prosperity(val: i32, scope: Province) {}
    fn add_province_modifier(name: name, The event modifier to add., duration: days, Optional. The number of days to add the province modifier for., <code>hidden = yes</code>, Optional. Whether the province modifier is hidden in the province view., desc: string, Optional. The string used to override the automatic duration string.) {}
    fn add_province_triggered_modifier(<code><modifier></code>, The province triggered modifier to add.<br>) {}
    fn add_reform_center(religion: (), scope: Province) {}
    fn add_scaled_local_adm_power(val: i32, scope: Province) {}
    fn add_scaled_local_dip_power(val: i32, scope: Province) {}
    fn add_scaled_local_mil_power(val: i32, scope: Province) {}
    fn add_siberian_construction(val: i32, scope: Province) {}
    fn add_territorial_core(scope: (), scope: Province) {}
    fn add_trade_modifier(who: scope, The country scope that receives the modifier., duration: days, The duration of the modifier., power: float, The amount of trade power to add or subtract., key: string, The key to display as the modifier in the tooltip.<br>) {}
    fn add_trade_node_income(val: i32, scope: Province) {}
    fn add_unit_construction(type: type, Which type of unit to build., amount: int, The amount to build., speed: float, The speed percentage of the base unit construction speed to use., cost: float, The cost percentage of the base unit construction cost to use., <code>mercenary = yes</code>, Optional. Makes the constructed units mercenaries.<br>) {}
    fn add_unrest(val: i32, scope: Province) {}
    fn artillery(scope: (), scope: Province) {}
    fn back_current_issue(<code>yes / no</code>, Boolean.) {}
    fn build_to_forcelimit(<code><type> = <float></code>, The type is any of the base unit types. The value is the percentage of forcelimit.) {}
    fn cancel_construction(<code>yes</code>, Boolean.) {}
    fn cavalry(scope: (), scope: Province) {}
    fn cede_province(scope: (), scope: Province) {}
    fn center_of_trade(<code><int></code>, Center of trade level.<br>) {}
    fn change_controller(scope: (), scope: Province) {}
    fn change_culture(culture: (), scope: Province) {}
    fn change_native_ferocity(val: i32, scope: Province) {}
    fn change_native_hostileness(val: i32, scope: Province) {}
    fn change_native_size(val: i32, scope: Province) {}
    fn change_province_name(string: (), scope: Province) {}
    fn change_religion(<code><religion></code>, The religion to change to., <code><scope></code>, The religion to change to, derived from the scope.) {}
    fn change_siege(val: i32, scope: Province) {}
    fn change_to_secondary_religion(<code>yes</code>, Boolean.) {}
    fn change_trade_goods(good: (), scope: Province) {}
    fn clr_province_flag(flag: (), scope: Province) {}
    fn cossack_cavalry(scope: (), scope: Province) {}
    fn create_advisor(advisor: (), scope: Province) {}
    fn create_colony(val: i32, scope: Province) {}
    fn create_native(val: i32, scope: Province) {}
    fn create_pirate(val: i32, scope: Province) {}
    fn create_revolt(val: i32, scope: Province) {}
    fn discover_country(scope: (), scope: Province) {}
    fn extend_province_modifier(name: name, The event modifier to extend., duration: days<br>) {}
    fn galley(scope: (), scope: Province) {}
    fn heavy_ship(scope: (), scope: Province) {}
    fn infantry(scope: (), scope: Province) {}
    fn kill_leader(type: (), scope: Province) {}
    fn kill_units(who: scope, Optional. Which country scopes should be affected., type: type, Optional. Which type of units that should be affected., amount: int, Optional. The specific amount to kill.) {}
    fn light_ship(scope: (), scope: Province) {}
    fn mercenary_cavalry(scope: (), scope: Province) {}
    fn mercenary_infantry(scope: (), scope: Province) {}
    fn multiply_colonysize(float: (), scope: Province) {}
    fn province_event(id: id, The event id to fire., days: days, Optional. The number of days to wait after the effect is executed to fire the event., random: days, Optional. The max number of additional days that can be added to the ''days'' parameter for randomness., tooltip: string, Optional. The tooltip to display in the effect tooltip ''country_event'' is used in.) {}
    fn recall_merchant(scope: (), scope: Province) {}
    fn remove_building(building: (), scope: Province) {}
    fn remove_cardinal(<code>yes</code>, Boolean.) {}
    fn remove_claim(scope: (), scope: Province) {}
    fn remove_core(scope: (), scope: Province) {}
    fn remove_estate(estate: (), scope: Province) {}
    fn remove_loot(who: scope, The country scope that receives the loot., amount: int, The amount of loot to take.) {}
    fn remove_province_modifier(modifier: (), scope: Province) {}
    fn remove_province_triggered_modifier(<code><modifier></code>, The province triggered modifier to remove.<br>) {}
    fn remove_reform_center(religion: (), scope: Province) {}
    fn remove_territorial_core(scope: (), scope: Province) {}
    fn remove_trade_modifier(who: scope, The country scope that has the trade modifier., name: string, The trade modifier to remove.<br>) {}
    fn rename_capital(string: (), scope: Province) {}
    fn send_missionary(<code>yes</code>, Boolean.) {}
    fn set_estate(estate: (), scope: Province) {}
    fn set_in_empire(<code>yes / no</code>, Boolean.) {}
    fn set_local_autonomy(val: i32, scope: Province) {}
    fn set_province_flag(flag: (), scope: Province) {}
    fn set_seat_in_parliament(<code>yes / no</code>, Boolean.) {}
    fn spawn_rebels(type: type, The size type to use., size: int, The size to use., <code>culture = <culture> / <scope></code>, Optional. The culture the rebels belong to., <code>religion = <religion> / <scope></code>, Optional. The religion the rebels belong to., unrest: int, Optional. Adds this amount of unrest to the province the rebels spawn in., <code>win = yes</code>, Optional. Changes control of the province the rebels spawn in to the rebels., friend: scope, Optional. Associate a country with the rebels., leader: string, Optional. Assigns the rebel leader a specific name. Can use ''set_saved_name''., <code>female = yes</code>, Optional. Makes the rebel leader female., <code>use_heir_as_leader = yes</code>, Optional. Makes the owner's heir the rebel leader. Removes heir., <code>use_consort_as_leader = yes</code>, Optional. Makes the owner's consort the rebel leader. Removes consort.) {}
    fn streltsy_infantry(scope: (), scope: Province) {}
    fn transport(scope: (), scope: Province) {}
    fn undiscover_country(scope: (), scope: Province) {}
*/
}
