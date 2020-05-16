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

    /*
|- @@@@@ <unit name> @@@@@ <code><scope></code>, The province scope in which the unit spawns. @@@@@ <pre>random_owned_province = {
    fn add_absolutism(int: (), scope: Country) {}
    fn add_accepted_culture(culture: String, scope: Country) {}
    fn add_active_policy(policy: (), scope: Country) {}
    fn add_adm_power(int: (), scope: Country) {}
    fn add_adm_tech(int: (), scope: Country) {}
    fn add_army_professionalism(float: (), scope: Country) {}
    fn add_army_tradition(int: (), scope: Country) {}
    fn add_authority(int: (), scope: Country) {}
    fn add_casus_belli(target: String, type: String, months: i32, scope: Country) {}
    fn add_church_aspect(aspect: (), scope: Country) {}
    fn add_church_power(int: (), scope: Country) {}
    fn add_corruption(float: (), scope: Country) {}*/
    fn add_country_modifier(name: String, duration: i32, /* hidden: bool, desc: String */ scope: Country) {}
    /*fn add_devotion(int: (), scope: Country) {}
    fn add_dip_power(int: (), scope: Country) {}
    fn add_dip_tech(int: (), scope: Country) {}
    fn add_disaster_modifier(<code>name = <modifier></code>, The event modifier to add., <code>disaster = <disaster></code>, The disaster to apply the modifier to., <code>duration = <days></code>, The duration for the modifier to be active for.) {}
    fn add_disaster_progress(<code>disaster = <disaster></code>, The disaster to alter., <code>value = <int></code>, The amount of progress to add or subtract.) {}
    fn add_doom(int: (), scope: Country) {}
    fn add_estate_influence_modifier(<code>estate = <estate></code>, The estate to apply the modifier to., <code>desc = <string></code>, The text to display in the Influence tooltip for this modifier., <code>influence = <int></code>, The amount of influence to add or subtract., <code>duration = <days></code>, The duration for which this modifier is active.<br>) {}
    fn add_estate_loyalty(<code>estate = <estate></code>, The estate to apply the loyalty to., <code>loyalty = <int></code>, The amount of loyalty to add or subtract.<br>) {}
    fn add_estate_loyalty_modifier(<code>estate = <estate></code>, The estate to apply the modifier to., <code>desc = <string></code>, The text to display in the Influence tooltip for this modifier., <code>loyalty = <int></code>, The amount of loyalty to add or subtract., <code>duration = <days></code>, The duration for which this modifier is active.<br>) {}
    fn add_faction(faction: (), scope: Country) {}
    fn add_faction_influence(<code>faction = <faction></code>, The faction to add influence to., <code>influence = <int></code>, The amount of influence to add.) {}
    fn add_fervor(int: (), scope: Country) {}
    fn add_government_power(<code>government_mechanic = <type></code>, Which government mechanic to add power to., <code>which = <type></code>, Which pool type to add the power to. ''ADM, DIP or MIL'', <code>amount = <type></code>, The amount of power to add.) {}
    fn add_government_reform(reform: (), scope: Country) {}
    fn add_harmonization_progress(int: (), scope: Country) {}
    fn add_harmonized_religion(religion: (), scope: Country) {}
    fn add_harmony(int: (), scope: Country) {}
    fn add_heir_claim(int: (), scope: Country) {}
    fn add_heir_personality(personality: (), scope: Country) {}
    fn add_heir_support(int: (), scope: Country) {}
    fn add_historical_friend(scope: (), scope: Country) {}
    fn add_historical_rival(scope: (), scope: Country) {}
    fn add_horde_unity(<code><int></code>, The amount of horde unity to add or subtract.<br>) {}
    fn add_idea(idea: (), scope: Country) {}
    fn add_idea_group(ideagroup: (), scope: Country) {}
    fn add_imperial_influence(int: (), scope: Country) {}
    fn add_incident_variable_value(<code>incident = <incident></code>, The incident to change., <code>value = <int></code>, The amount to add to the incident variable.) {}
    fn add_inflation(float: (), scope: Country) {}
    fn add_isolationism(int: (), scope: Country) {}
    fn add_karma(int: (), scope: Country) {}
    fn add_legitimacy(<code><int></code>, The amount of legitimacy to add or subtract., <code><scope></code>, The amount of legitimacy to add or subtract, taking the value from the scope's current value.<br>) {}
    fn add_liberty_desire(int: (), scope: Country) {}
|- @@@@@ add_loan @@@@@ <code>interest_modifier = <value></code>, <code>fixed_interest = <boolean></code>, <code>duration = <integer></code>
    fn add_mandate(int: (), scope: Country) {}
    fn add_manpower(float: (), scope: Country) {}
    fn add_mercantilism(int: (), scope: Country) {}
    fn add_meritocracy(<code><int></code>, The amount of meritocracy to add or subtract.<br>) {}
    fn add_mil_power(int: (), scope: Country) {}
    fn add_mil_tech(int: (), scope: Country) {}
    fn add_militarised_society(<code><int></code>, The amount of militarisation to add or subtract.<br>) {}
    fn add_navy_tradition(int: (), scope: Country) {}
    fn add_next_institution_embracement(int: (), scope: Country) {}
    fn add_opinion(<code>who = <scope></code>, The country the opinion modifier is for., <code>modifier = <modifier></code>, The opinion modifier to use., <code>years = <int></code>, Optional. Whether the modifier expires after a set amount of years.) {}
    fn add_papal_influence(int: (), scope: Country) {}
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
    fn add_ruler_modifier(<code>name = <name></code>, The event modifier to add., <code>duration = <days></code>, Optional. The number of days to add the ruler modifier for., <code>hidden = yes</code>, Optional. Whether the ruler modifier is hidden in the government view., <code>desc = <string</code>, Optional. The string used to override the automatic duration string.) {}
    fn add_ruler_personality(personality: (), scope: Country) {}
    fn add_sailors(int: (), scope: Country) {}
    fn add_scaled_imperial_influence(int: (), scope: Country) {}
    fn add_scaled_republican_tradition(<code><int></code>, The amount of republican tradition to add or subtract.<br>) {}
    fn add_splendor(int: (), scope: Country) {}
    fn add_spy_network_from(<code>who = <scope></code>, The country to add spy network for., <code>value = <int></code>, The amount of spy network to add or subtract.) {}
    fn add_spy_network_in(<code>who = <scope></code>, The country to add spy network in., <code>value = <int></code>, The amount of spy network to add or subtract.) {}*/
    fn add_stability(val: i32, scope: Country) {}
    /*fn add_tariff_value(float: (), scope: Country) {}
    fn add_treasury(int: (), scope: Country) {}
    fn add_tribal_allegiance(<code><int></code>, The amount of tribal allegiance to add or subtract.<br>) {}
    fn add_truce_with(scope: (), scope: Country) {}
    fn add_trust(<code>who = <scope></code>, The country trust will be added with., <code>value = <int></code>, The amount of trust to add or subtract., <code>mutual = yes</code>, Optional. Whether to apply the trust change to both countries.) {}
    fn add_war_exhaustion(int: (), scope: Country) {}
    fn add_yearly_manpower(float: (), scope: Country) {}
    fn add_yearly_sailors(float: (), scope: Country) {}
    fn add_years_of_income(float: (), scope: Country) {}
|- @@@@@ adopt_reform_progress  @@@@@ <code><country scope></code>, Adopts the government reform progress from the target country scope.<br> @@@@@ <code>adopt_reform_progress = FROM</code> @@@@@ Adopts the government reform progress from the target country scope. @@@@@ Used when the papal states is released. @@@@@ ???
    fn artillery(scope: (), scope: Country) {}
    fn break_marriage(scope: (), scope: Country) {}
    fn break_union(scope: (), scope: Country) {}
    fn cavalry(scope: (), scope: Country) {}
    fn change_adm(int: (), scope: Country) {}
    fn change_consort_regent_to_ruler(<code>yes</code>, Boolean.) {}
    fn change_dip(int: (), scope: Country) {}
    fn change_government(government: (), scope: Country) {}
    fn change_government_reform_progress(int: (), scope: Country) {}
    fn change_graphical_culture(gfxculture: (), scope: Country) {}
    fn change_heir_adm(int: (), scope: Country) {}
    fn change_heir_dip(int: (), scope: Country) {}
    fn change_heir_mil(int: (), scope: Country) {}
|- @@@@@ change_innovativeness @@@@@ <code><int></code>
    fn change_mil(int: (), scope: Country) {}
    fn change_personal_deity(deity: (), scope: Country) {}
    fn change_price(<code>trade_goods = <tradegood></code>, The trade good to change., <code>key = <string></code>, The localisation key to display in the Trade Good's price tooltip., <code>value = <float></code>, The percentage to change the value by., <code>duration = <days></code>, The duration for the value to stay changed for.) {}
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
    fn collapse_nation(<code>yes</code>, Boolean) {}
|- @@@@@ complete_mission @@@@@ <code><key></code>
    fn convert_female_heir_to_general(<code>yes</code>, Boolean.) {}
    fn convert_female_ruler_to_general(<code>yes</code>, Boolean.) {}
    fn convert_heir_to_general(<code>yes</code>, Boolean.) {}
    fn convert_ruler_to_general(<code>yes</code>, Boolean.) {}
|- @@@@@ country_event @@@@@ <code>id = <id></code>, <code>days = <days></code>, <code>random = <days></code>, <code>tooltip = <string></code>
|- @@@@@ create_admiral @@@@@ <code>tradition = <int></code>,<code>add_fire = <int></code>,<code>add_shock = <int></code>,<code>add_manuever = <int></code>,<code>add_siege = <int></code>,<code>culture = <identifier></code>
    fn create_advisor(advisor: (), scope: Country) {}
    fn create_alliance(scope: (), scope: Country) {}
    fn create_conquistador(int: (), scope: Country) {}
    fn create_explorer(int: (), scope: Country) {}
|- @@@@@ create_general @@@@@ <code>tradition = <int></code>,<code>add_fire = <int></code>,<code>add_shock = <int></code>,<code>add_manuever = <int></code>,<code>add_siege = <int></code>,<code>culture = <identifier></code>
    fn create_guarantee(scope: (), scope: Country) {}
    fn create_independent_estate(estate: (), scope: Country) {}
    fn create_independent_estate_from_religion(estate: (), scope: Country) {}
    fn create_march(scope: (), scope: Country) {}
    fn create_marriage(scope: (), scope: Country) {}
    fn create_subject(<code>subject_type = <type></code>, The subject type to use., <code>subject = <scope></code>, The scope to make the subject.) {}
    fn create_union(scope: (), scope: Country) {}
    fn create_vassal(scope: (), scope: Country) {}
    fn declare_war(scope: (), scope: Country) {}
    fn declare_war_with_cb(<code>who = <scope></code>, The country declared against., <code>casus_belli = <cb></code>, The casus belli to use., <code>war_goal_province = <province id></code>, Optional. Defines the province the war was declared for, if required by the casus belli.<br>) {}
    fn define_admiral(<code>shock = <int></code>, The shock pips of the leader., <code>fire = <int></code>, The fire pips of the leader., <code>manuever = <int></code>, The maneuver pips of the leader., <code>siege = <int></code>, The siege pips of the leader., <code>name = <string></code>, Optional. The name of the leader., <code>female = yes</code>, Optional. Makes the leader female., <code>trait = <trait></code>, Optional. A trait automatically assigned to the leader.) {}
    fn define_advisor(<code>type = <type></code>, The advisor type to create., <code>skill = <int></code>, The amount of skill the advisor has. Limited to 3 maximum., <code>name = <string></code>, Optional. The name to use., <code>location = <province id></code>, Optional. The province that the advisor comes from., <code>discount = yes</code>, Optional. If the SCRIPTED_ADVISOR_DISCOUNT cost reduction applies to this advisor., <code>female = yes</code>, Optional. Makes this advisor female., <code>culture = <culture> / <scope></code>, Optional. The culture of this advisor., <code>religion = <religion> / <scope></code>, Optional. The religion of this advisor.) {}
    fn define_conquistador(<code>shock = <int></code>, The shock pips of the leader., <code>fire = <int></code>, The fire pips of the leader., <code>manuever = <int></code>, The maneuver pips of the leader., <code>siege = <int></code>, The siege pips of the leader., <code>name = <string></code>, Optional. The name of the leader., <code>female = yes</code>, Optional. Makes the leader female., <code>trait = <trait></code>, Optional. A trait automatically assigned to the leader.) {}
    fn define_consort(<code>name = <string></code>, Optional. The name of the consort., <code>country_of_origin = <scope></code>, Optional. The origin country of the consort., <code>dynasty = <string> / <scope> / original_dynasty</code>, Optional. The dynasty of the consort., <code>age = <years></code>, Optional. The age of the consort., <code>adm = <int></code>, Optional. The ADM skill for the consort., <code>dip = <int></code>, Optional. The DIP skill for the consort., <code>mil = <int></code>, Optional. The MIL skill for the consort., <code>hide_skills = yes</code>, Optional. Hides the skill values for this consort., <code>female = yes</code>, Optional. Forces the consort to be female., <code>male = yes</code>, Optional. Forces the consort to be male., <code>culture = <culture> / <scope></code>, Optional. The culture of the consort., <code>religion = <religion> / <scope></code>, Optional. The religion of the consort.<br>) {}
    fn define_explorer(<code>shock = <int></code>, The shock pips of the leader., <code>fire = <int></code>, The fire pips of the leader., <code>manuever = <int></code>, The maneuver pips of the leader., <code>siege = <int></code>, The siege pips of the leader., <code>name = <string></code>, Optional. The name of the leader., <code>female = yes</code>, Optional. Makes the leader female., <code>trait = <trait></code>, Optional. A trait automatically assigned to the leader.) {}
    fn define_general(<code>shock = <int></code>, The shock pips of the leader., <code>fire = <int></code>, The fire pips of the leader., <code>manuever = <int></code>, The maneuver pips of the leader., <code>siege = <int></code>, The siege pips of the leader., <code>name = <string></code>, Optional. The name of the leader., <code>female = yes</code>, Optional. Makes the leader female., <code>trait = <trait></code>, Optional. A trait automatically assigned to the leader.) {}
    fn define_heir(<code>name = <string></code>, Optional. The name of the heir., <code>dynasty = <string> / <scope> / original_dynasty</code>, Optional. The dynasty of the heir., <code>age = <years></code>, Optional. The age of the heir., <code>birth_date = <date></code>, Optional. The birth date of the heir. Can be used instead of the ''age'' parameter., <code>claim = <int></code>, Optional. The starting legitimacy of the heir., <code>adm = <int></code>, Optional. The minimum ADM skill roll for the heir., <code>dip = <int></code>, Optional. The minimum DIP skill roll for the heir., <code>mil = <int></code>, Optional. The minimum MIL skill roll for the heir., <code>fixed = yes</code>, Optional. Makes the heir skill roll fixed to the defined minimums., <code>max_random_adm = <int></code>, Optional. The maximum ADM skill roll for the heir., <code>max_random_dip = <int></code>, Optional. The maximum DIP skill roll for the heir., <code>max_random_mil = <int></code>, Optional. The maximum MIL skill roll for the heir., <code>hide_skills = yes</code>, Optional. Hides the skill values for this heir.,<code>hidden = yes</code>, Optional. Hides the skill values for this heir. Legacy version of ''hide_skills''.,<code>no_consort_with_heir = yes</code>, Optional. Makes the heir a child not of the consort, i.e. a bastard,<code>female = yes</code>, Optional. Forces the heir to be female., <code>male = yes</code>, Optional. Forces the heir to be male., <code>culture = <culture> / <scope></code>, Optional. The culture of the heir., <code>religion = <religion> / <scope></code>, Optional. The religion of the heir.<br>) {}
    fn define_heir_to_general(<code>shock = <int></code>, The shock pips of the leader., <code>fire = <int></code>, The fire pips of the leader., <code>manuever = <int></code>, The maneuver pips of the leader., <code>siege = <int></code>, The siege pips of the leader.<br>) {}
    fn define_leader_to_ruler(<code>type = general / conquistador / admiral / explorer</code>, The leader type to pick a leader from., <code>name = <string></code>, The specific leader to use. Used instead of the ''type'' parameter., <code>claim = <int></code>, Optional. The starting legitimacy of the ruler., <code>adm = <int></code>, Optional. The minimum ADM skill roll for the ruler., <code>dip = <int></code>, Optional. The minimum DIP skill roll for the ruler., <code>mil = <int></code>, Optional. The minimum MIL skill roll for the ruler., <code>fixed = yes</code>, Optional. Makes the ruler skill roll fixed to the defined minimums.<br>) {}
|- @@@@@ define_ruler @@@@@ <code>name = <string></code>, Optional. The name of the ruler., <code>dynasty = <string> / <scope> / original_dynasty</code>, Optional. The dynasty of the ruler., <code>age = <years></code>, Optional. The age of the ruler., <code>claim = <int></code>, Optional. The starting legitimacy of the ruler., <code>adm = <int></code>, Optional. The minimum ADM skill roll for the ruler., <code>dip = <int></code>, Optional. The minimum DIP skill roll for the ruler., <code>mil = <int></code>, Optional. The minimum MIL skill roll for the ruler., <code>fixed = yes</code>, Optional. Makes the ruler skill roll fixed to the defined minimums., <code>max_random_adm = <int></code>, Optional. The maximum ADM skill roll for the ruler., <code>max_random_dip = <int></code>, Optional. The maximum DIP skill roll for the ruler., <code>max_random_mil = <int></code>, Optional. The maximum MIL skill roll for the ruler., <code>hide_skills = yes</code>, Optional. Hides the skill values for this ruler.
    fn define_ruler_to_general(<code>shock = <int></code>, The shock pips of the leader., <code>fire = <int></code>, The fire pips of the leader., <code>manuever = <int></code>, The maneuver pips of the leader., <code>siege = <int></code>, The siege pips of the leader.<br>) {}
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
    fn kill_leader(<code>type = <type></code>, The leader type to kill. ''name'' is the specific leader name to target.) {}
    fn kill_ruler(<code>yes</code>, Boolean.) {}
    fn leave_league(scope: (), scope: Country) {}
    fn light_ship(scope: (), scope: Country) {}
    fn loan_size(int: (), scope: Country) {}
    fn mercenary_cavalry(scope: (), scope: Country) {}
    fn mercenary_infantry(scope: (), scope: Country) {}
    fn reinstate_parliament(<code>yes</code>, Boolean.) {}
    fn release(scope: (), scope: Country) {}
    fn release_all_subjects(<code>yes</code>, Boolean.) {}
    fn remove_accepted_culture(<code><culture></code>, The culture to remove, from {{path|common/cultures/}}, <code><scope></code>, The culture to remove, derived from the scope.) {}
    fn remove_advisor(<code><random></code>, Remove a random advisor., <code><advisor></code>, Remove a specific advisor type, <code><advisor id></code>, Remove a specific advisor id.<br>) {}
    fn remove_advisor_by_category(type: (), scope: Country) {}
    fn remove_casus_belli(<code>target = <scope></code>, The country the casus belli is against., <code>type = <cb></code>, The casus belli to remove.<br>) {}
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
    fn remove_opinion(<code>who = <scope></code>, The country the opinion modifier is held against., <code>modifier = <modifier></code>, The opinion modifier to use.<br>) {}
    fn remove_queen_personality(personality: (), scope: Country) {}
    fn remove_religious_reforms(int: (), scope: Country) {}
    fn remove_ruler_personality(personality: (), scope: Country) {}
    fn reverse_add_casus_belli(<code>target = <scope></code>, The country to that gains the casus belli., <code>type = <cb></code>, The casus belli to add., <code>months = <months></code>, The number of months the casus belli lasts for.) {}
    fn reverse_add_opinion(<code>who = <scope></code>, The country the opinion modifier is added to., <code>modifier = <modifier></code>, The opinion modifier to use., <code>years = <int></code>, Optional. Whether the modifier expires after a set amount of years.) {}
    fn reverse_remove_casus_belli(<code>target = <scope></code>, The country that holds the casus belli., <code>type = <cb></code>, The casus belli to remove.<br>) {}
    fn reverse_remove_opinion(<code>who = <scope></code>, The country the opinion modifier holds., <code>modifier = <modifier></code>, The opinion modifier to use.<br>) {}
    fn revoke_reform(reform: (), scope: Country) {}
    fn set_ai_attitude(<code>attitude = <type></code>, The attitude to use., <code>who = <scope></code>, Who the attitude is directed against., <code>locked = <yes> / <no></code>, Whether the attitude can be re-evaluated and changed.) {}
    fn set_ai_personality(<code>personality = <type></code>, The personality to use., <code>locked = <yes> / <no></code>, Whether the personality can be re-evaluated and changed.) {}
    fn set_allow_female_emperor(<code>yes</code>, Boolean.) {}
    fn set_consort_culture(<code><culture></code>, The culture to change to.<br>) {}
    fn set_consort_flag(flag: (), scope: Country) {}
    fn set_consort_religion(<code><religion></code>, The religion to change to.<br>) {}*/
    fn set_country_flag(flag: String, scope: Country) {}
    /*fn set_dynasty(<code><string> / <scope> / original_dynasty</code>, The dynasty to set.<br>) {}
    fn set_emperor_of_china(scope: (), scope: Country) {}
    fn set_government_and_rank(<code>government = <government></code>, The government to change to., <code>rank = <int></code>, The rank to change to.) {}
    fn set_government_rank(int: (), scope: Country) {}
    fn set_heir(string: (), scope: Country) {}
    fn set_heir_culture(<code><culture></code>, The culture to change to.<br>) {}
    fn set_heir_flag(flag: (), scope: Country) {}
    fn set_heir_religion(<code><religion></code>, The religion to change to.<br>) {}
    fn set_hre_heretic_religion(religion: (), scope: Country) {}
    fn set_hre_religion(religion: (), scope: Country) {}
    fn set_hre_religion_locked(<code>yes</code>, Boolean.) {}
    fn set_hre_religion_treaty(<code>yes</code>, Boolean.) {}
    fn set_in_empire(<code>yes</code>, Boolean.) {}
    fn set_incident_variable_value(<code>incident = <incident></code>, The incident to change., <code>value = <int></code>, The amount to set the incident variable to.) {}
    fn set_isolationism(int: (), scope: Country) {}
    fn set_karma(int: (), scope: Country) {}
    fn set_legacy_government(<code>government = <government></code>, The pre-Dharma government type to change to.) {}
    fn set_mandate(int: (), scope: Country) {}
    fn set_meritocracy(<code><int></code>, The amount of meritocracy to set.<br>) {}
    fn set_papacy_active(<code>yes</code><code>no</code>, Boolean.) {}
    fn set_primitive(<code><yes><no></code>, Boolean.) {}
    fn set_revolution_target(scope: (), scope: Country) {}
    fn set_ruler(string: (), scope: Country) {}
    fn set_ruler_culture(<code><culture></code>, The culture to change to.<br>) {}
    fn set_ruler_flag(flag: (), scope: Country) {}
    fn set_ruler_religion(<code><religion></code>, The religion to change to.<br>) {}
    fn set_saved_name(<code>key = <string></code>, The key that holds the name., <code>type = advisor / simple</code>, The name generation type. Advisor draws from namelist, simple utilises name parameter., <code>name = <string></code>, The name to hold in the key. Only used with the ''simple'' type., <code>scope = <scope></code>, The country scope to draw the namelist from. Only used with the ''advisor'' type.<br>) {}
    fn set_school_opinion(<code>who = <who></code>, Which country to alter opinion with., <code>opinion = <opinion></code>, Which opinion state to change to.<br>) {}
    fn swap_free_idea_group(yes: (), scope: Country) {}
    fn switch_tag(scope: (), scope: Country) {}
    fn transport(scope: (), scope: Country) {}
    fn unlock_cult(cult: (), scope: Country) {}
    fn vassalize(scope: (), scope: Country) {}
    fn white_peace(scope: (), scope: Country) {}

/////// PROVINCE SCOPE //////
|- @@@@@ <rebel type> @@@@@ <code><int></code>, The size to use. @@@@@ <code>anti_tax_rebels = 1</code>
|- @@@@@ <unit name> @@@@@ <code><scope></code>, The country scope the unit belongs to @@@@@ <pre>capital_scope = {
    fn add_base_manpower(int: (), scope: Province) {}
    fn add_base_production(int: (), scope: Province) {}
    fn add_base_tax(int: (), scope: Province) {}
    fn add_building(building: (), scope: Province) {}
    fn add_building_construction(<code>building = <building></code>, The building to construct., <code>speed = <float></code>, The speed percentage of the base building speed to use., <code>cost = <float></code>, The cost percentage of the base building cost to use.<br>) {}
    fn add_cardinal(<code>yes</code>, Boolean.) {}
    fn add_center_of_trade_level(<code><int></code>, Amount to increase center of trade level.<br>) {}
    fn add_claim(scope: (), scope: Province) {}
    fn add_colonysize(int: (), scope: Province) {}
    fn add_construction_progress(float: (), scope: Province) {}
    fn add_core(scope: (), scope: Province) {}
    fn add_core_construction(<code>yes</code><br> Boolean., <code>speed = <float></code>, The speed percentage of the base coring speed to use., <code>cost = <float></code>, The cost percentage of the base coring cost to use.) {}
    fn add_culture_construction(<code>yes</code>, Boolean., <code>speed = <float></code>, The speed percentage of the base conversion speed to use., <code>cost = <float></code>, The cost percentage of the base conversion cost to use.<br>) {}
    fn add_devastation(int: (), scope: Province) {}
    fn add_great_project(project: (), scope: Province) {}
    fn add_institution_embracement(<code>which = <institution></code>, The institution to add to., <code>value = <int></code>, The amount of embracement to add or subtract.) {}
    fn add_local_autonomy(int: (), scope: Province) {}
    fn add_nationalism(int: (), scope: Province) {}
    fn add_permanent_claim(scope: (), scope: Province) {}
    fn add_permanent_province_modifier(<code>name = <name></code>, The event modifier to add., <code>duration = <days></code>, Optional. The number of days to add the province modifier for., <code>hidden = yes</code>, Optional. Whether the province modifier is hidden in the province view., <code>desc = <string></code>, Optional. The string used to override the automatic duration string.) {}
    fn add_prosperity(int: (), scope: Province) {}
    fn add_province_modifier(<code>name = <name></code>, The event modifier to add., <code>duration = <days></code>, Optional. The number of days to add the province modifier for., <code>hidden = yes</code>, Optional. Whether the province modifier is hidden in the province view., <code>desc = <string></code>, Optional. The string used to override the automatic duration string.) {}
    fn add_province_triggered_modifier(<code><modifier></code>, The province triggered modifier to add.<br>) {}
    fn add_reform_center(religion: (), scope: Province) {}
    fn add_scaled_local_adm_power(int: (), scope: Province) {}
    fn add_scaled_local_dip_power(int: (), scope: Province) {}
    fn add_scaled_local_mil_power(int: (), scope: Province) {}
    fn add_siberian_construction(int: (), scope: Province) {}
    fn add_territorial_core(scope: (), scope: Province) {}
    fn add_trade_modifier(<code>who = <scope></code>, The country scope that receives the modifier., <code>duration = <days></code>, The duration of the modifier., <code>power = <float></code>, The amount of trade power to add or subtract., <code>key = <string></code>, The key to display as the modifier in the tooltip.<br>) {}
    fn add_trade_node_income(int: (), scope: Province) {}
    fn add_unit_construction(<code>type = <type></code>, Which type of unit to build., <code>amount = <int></code>, The amount to build., <code>speed = <float></code>, The speed percentage of the base unit construction speed to use., <code>cost = <float></code>, The cost percentage of the base unit construction cost to use., <code>mercenary = yes</code>, Optional. Makes the constructed units mercenaries.<br>) {}
    fn add_unrest(int: (), scope: Province) {}
    fn artillery(scope: (), scope: Province) {}
    fn back_current_issue(<code>yes / no</code>, Boolean.) {}
    fn build_to_forcelimit(<code><type> = <float></code>, The type is any of the base unit types. The value is the percentage of forcelimit.) {}
    fn cancel_construction(<code>yes</code>, Boolean.) {}
    fn cavalry(scope: (), scope: Province) {}
    fn cede_province(scope: (), scope: Province) {}
    fn center_of_trade(<code><int></code>, Center of trade level.<br>) {}
    fn change_controller(scope: (), scope: Province) {}
    fn change_culture(culture: (), scope: Province) {}
    fn change_native_ferocity(int: (), scope: Province) {}
    fn change_native_hostileness(int: (), scope: Province) {}
    fn change_native_size(int: (), scope: Province) {}
    fn change_province_name(string: (), scope: Province) {}
    fn change_religion(<code><religion></code>, The religion to change to., <code><scope></code>, The religion to change to, derived from the scope.) {}
    fn change_siege(int: (), scope: Province) {}
    fn change_to_secondary_religion(<code>yes</code>, Boolean.) {}
    fn change_trade_goods(good: (), scope: Province) {}
    fn clr_province_flag(flag: (), scope: Province) {}
    fn cossack_cavalry(scope: (), scope: Province) {}
    fn create_advisor(advisor: (), scope: Province) {}
    fn create_colony(int: (), scope: Province) {}
    fn create_native(int: (), scope: Province) {}
    fn create_pirate(int: (), scope: Province) {}
    fn create_revolt(int: (), scope: Province) {}
    fn discover_country(scope: (), scope: Province) {}
    fn extend_province_modifier(<code>name = <name></code>, The event modifier to extend., <code>duration = <days></code><br>) {}
    fn galley(scope: (), scope: Province) {}
    fn heavy_ship(scope: (), scope: Province) {}
    fn infantry(scope: (), scope: Province) {}
    fn kill_leader(type: (), scope: Province) {}
    fn kill_units(<code>who = <scope></code>, Optional. Which country scopes should be affected., <code>type = <type></code>, Optional. Which type of units that should be affected., <code>amount = <int></code>, Optional. The specific amount to kill.) {}
    fn light_ship(scope: (), scope: Province) {}
    fn mercenary_cavalry(scope: (), scope: Province) {}
    fn mercenary_infantry(scope: (), scope: Province) {}
    fn multiply_colonysize(float: (), scope: Province) {}
    fn province_event(<code>id = <id></code>, The event id to fire., <code>days = <days></code>, Optional. The number of days to wait after the effect is executed to fire the event., <code>random = <days></code>, Optional. The max number of additional days that can be added to the ''days'' parameter for randomness., <code>tooltip = <string></code>, Optional. The tooltip to display in the effect tooltip ''country_event'' is used in.) {}
    fn recall_merchant(scope: (), scope: Province) {}
    fn remove_building(building: (), scope: Province) {}
    fn remove_cardinal(<code>yes</code>, Boolean.) {}
    fn remove_claim(scope: (), scope: Province) {}
    fn remove_core(scope: (), scope: Province) {}
    fn remove_estate(estate: (), scope: Province) {}
    fn remove_loot(<code>who = <scope></code>, The country scope that receives the loot., <code>amount = <int></code>, The amount of loot to take.) {}
    fn remove_province_modifier(modifier: (), scope: Province) {}
    fn remove_province_triggered_modifier(<code><modifier></code>, The province triggered modifier to remove.<br>) {}
    fn remove_reform_center(religion: (), scope: Province) {}
    fn remove_territorial_core(scope: (), scope: Province) {}
    fn remove_trade_modifier(<code>who = <scope></code>, The country scope that has the trade modifier., <code>name = <string></code>, The trade modifier to remove.<br>) {}
    fn rename_capital(string: (), scope: Province) {}
    fn send_missionary(<code>yes</code>, Boolean.) {}
    fn set_estate(estate: (), scope: Province) {}
    fn set_in_empire(<code>yes / no</code>, Boolean.) {}
    fn set_local_autonomy(int: (), scope: Province) {}
    fn set_province_flag(flag: (), scope: Province) {}
    fn set_seat_in_parliament(<code>yes / no</code>, Boolean.) {}
    fn spawn_rebels(<code>type = <type></code>, The size type to use., <code>size = <int></code>, The size to use., <code>culture = <culture> / <scope></code>, Optional. The culture the rebels belong to., <code>religion = <religion> / <scope></code>, Optional. The religion the rebels belong to., <code>unrest = <int></code>, Optional. Adds this amount of unrest to the province the rebels spawn in., <code>win = yes</code>, Optional. Changes control of the province the rebels spawn in to the rebels., <code>friend = <scope></code>, Optional. Associate a country with the rebels., <code>leader = <string></code>, Optional. Assigns the rebel leader a specific name. Can use ''set_saved_name''., <code>female = yes</code>, Optional. Makes the rebel leader female., <code>use_heir_as_leader = yes</code>, Optional. Makes the owner's heir the rebel leader. Removes heir., <code>use_consort_as_leader = yes</code>, Optional. Makes the owner's consort the rebel leader. Removes consort.) {}
    fn streltsy_infantry(scope: (), scope: Province) {}
    fn transport(scope: (), scope: Province) {}
    fn undiscover_country(scope: (), scope: Province) {}
*/
}
