use std::collections::HashMap;
use crate::Date;

pub type ReligionList = HashMap<String, ReligiousGroup>;
// XXX: This is really, really wrong.
pub type ProvinceRef = i32;

#[derive(ParadoxScope, Default, Debug)]
pub struct ReligiousGroup {
    // Inherited properties
    pub defender_of_faith: bool,
    pub can_form_personal_unions: bool,
    pub crusade_name: String, // XXX: l10n
    pub center_of_religion: ProvinceRef,
    pub ai_will_propagate_through_trade: bool,

    // XXX: graphics stuff
    pub flags_with_emblem_percentage: i32,
    pub flag_emblem_index_range: Vec<i32>,

    // Undocumented
    pub religious_schools: HashMap<String, ReligiousSchool>,

    #[collect_scopes]
    pub religions: Vec<Religion>
}

#[derive(ParadoxScope, Default, Debug)]
pub struct Religion {
    pub name: String,

    // Required
    pub color: Vec<i32>, // XXX: RGB
    pub icon: i32, // XXX: icon ref?
    pub heretic: Vec<String>,

    // More mechanics:
    pub papacy: bool,
    pub hre_religion: bool,
    pub hre_heretic_religion: bool,
    pub date: Date,
    pub fervor: bool,
    pub has_patriarchs: bool,
    pub uses_piety: bool,
    pub personal_deity: bool,
    pub authority: bool,
    pub religious_reforms: bool,
    pub doom: bool,
    pub uses_church_power: bool,
    pub uses_karma: bool,
    pub misguided_heretic: bool,
    pub declare_war_in_regency: bool,
    pub can_have_secondary_religion: bool,

    // Undocumented:
    pub uses_harmony: bool,
    pub uses_isolationism: bool,
    pub fetishist_cult: bool,
    pub uses_anglican_power: bool,
}

#[derive(ParadoxScope, Default, Debug)]
pub struct ReligiousSchool {
}
