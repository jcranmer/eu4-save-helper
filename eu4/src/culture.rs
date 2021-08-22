use crate::{Country, CountryModifier, ProvinceModifier};
use paradox::{IdKey, ParadoxParse, ParserAtom};
use std::collections::HashMap;

pub type CultureGroupList = HashMap<IdKey<CultureGroup>, CultureGroup>;

#[derive(ParadoxParse, Default)]
pub struct CultureGroup {
    #[optional] pub graphical_culture: String,
    #[optional] pub second_graphical_culture: String,
    #[optional] pub dynasty_names: Vec<String>,
    #[optional] pub female_names: Vec<String>,
    #[optional] pub male_names: Vec<String>,

    #[collect]
    pub cultures: HashMap<IdKey<Culture>, Culture>
}

#[derive(ParadoxParse, Default)]
pub struct Culture {
    #[optional] pub graphical_culture: String,
    #[optional] pub second_graphical_culture: String,
    #[optional] pub dynasty_names: Vec<String>,
    #[optional] pub female_names: Vec<String>,
    #[optional] pub male_names: Vec<String>,
    #[optional] pub primary: ParserAtom,
    
    #[optional] pub country: Vec<CountryModifier>,
    #[optional] pub province: Vec<ProvinceModifier>
}
