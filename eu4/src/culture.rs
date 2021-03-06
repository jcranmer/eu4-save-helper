use crate::{Country, Eu4Atom, Modifiers};
use paradox::ParadoxParse;
use std::collections::HashMap;

pub type CultureGroupList = HashMap<Eu4Atom, CultureGroup>;

#[derive(ParadoxParse, Default)]
pub struct CultureGroup {
    #[optional] pub graphical_culture: String,
    #[optional] pub second_graphical_culture: String,
    #[optional] pub dynasty_names: Vec<String>,
    #[optional] pub female_names: Vec<String>,
    #[optional] pub male_names: Vec<String>,

    #[collect]
    pub cultures: HashMap<Eu4Atom, Culture>
}

#[derive(ParadoxParse, Default)]
pub struct Culture {
    #[optional] pub graphical_culture: String,
    #[optional] pub second_graphical_culture: String,
    #[optional] pub dynasty_names: Vec<String>,
    #[optional] pub female_names: Vec<String>,
    #[optional] pub male_names: Vec<String>,
    #[optional] pub primary: Eu4Atom,
    
    #[optional] pub country: Modifiers,
    #[optional] pub province: Modifiers,
}
