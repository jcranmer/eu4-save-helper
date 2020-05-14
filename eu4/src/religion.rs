use crate::{LocalizationKey, ProvinceRef, RgbColor};
use paradox::{Date, ParadoxParse};
use std::collections::HashMap;

pub type ReligionList = HashMap<String, ReligiousGroup>;

#[derive(ParadoxParse, Default, Debug)]
pub struct ReligiousGroup {
    // Inherited properties
    #[optional]
    pub defender_of_faith: bool,
    #[optional]
    pub can_form_personal_unions: bool,
    pub crusade_name: LocalizationKey,
    #[optional]
    pub center_of_religion: ProvinceRef,
    #[optional]
    pub ai_will_propagate_through_trade: bool,

    #[optional]
    pub flags_with_emblem_percentage: u32,
    pub flag_emblem_index_range: [u32; 2],

    // Undocumented on the wiki.
    #[optional]
    pub harmonized_modifier: String, // XXX: EventModifier
    #[optional]
    pub religious_schools: HashMap<String, ReligiousSchool>,

    #[collect]
    pub religions: Vec<Religion>
}

#[derive(ParadoxParse, Default, Debug)]
pub struct Religion {
    pub color: RgbColor,
    pub icon: i32, // XXX: icon reference?
    pub heretic: Vec<String>,

    #[optional]
    pub flags_with_emblem_percentage: u32,
    #[optional]
    pub flag_emblem_index_range: [u32; 2],

    #[optional]
    pub allowed_conversion: Vec<String>, // XXX: ReligionRef

    #[optional]
    pub date: Date,

    #[optional]
    pub holy_sites: Vec<ProvinceRef>,

    #[optional] pub aspects_name: LocalizationKey,
    #[optional] pub reform_tooltip: LocalizationKey,

    // Mechanics:
    #[optional] pub authority: bool,
    #[optional] pub can_have_secondary_religion: bool,
    #[optional] pub declare_war_in_regency: bool,
    #[optional] pub doom: bool,
    #[optional] pub fervor: bool,
    #[optional] pub fetishist_cult: bool,
    #[optional] pub has_patriarchs: bool,
    #[optional] pub hre_heretic_religion: bool,
    #[optional] pub hre_religion: bool,
    #[optional] pub misguided_heretic: bool,
    #[optional] pub personal_diety: bool,
    #[optional] pub religious_reforms: bool,
    #[optional] pub uses_anglican_power: bool,
    #[optional] pub uses_church_power: bool,
    #[optional] pub uses_harmony: bool,
    #[optional] pub uses_isolationism: bool,
    #[optional] pub uses_karma: bool,
    #[optional] pub uses_piety: bool,
}

#[derive(ParadoxParse, Default, Debug)]
pub struct ReligiousSchool {
    pub can_invite_scholar: (),
    pub on_invite_scholar: (),
    pub potential_invite_scholar: (),
    pub invite_scholar_modifier_display: (),
    pub picture: String, // XXX: gfx reference or something?

    //#[effects]
    //pub effects: Vec<CountryEffect>
}

/*
#[derive(Default, Debug)]
pub struct CountryEffect {
    effect: &'static str,
}

impl std::convert::TryFrom<(String, paradox::UnparsedValue<'_>)> for CountryEffect {
    type Error = paradox::ParseError;

    fn try_from(kv_pair: (String, paradox::UnparsedValue<'_>)) -> Result<Self, paradox::ParseError> {
        match &kv_pair.0 {
            "adm_tech_cost_modifier" =>
        }
    }
}*/
