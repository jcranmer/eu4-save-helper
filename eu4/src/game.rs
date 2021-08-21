use paradox::{BoxedValue, IdKey, TypeDefinition};
use std::collections::HashMap;

#[derive(paradox::GameData)]
pub struct GameData {
    pub base_info: paradox::GameData,

    #[parse = "common/country_tags"]
    tags: crate::CountryMap,

    #[parse = "map/area.txt"]
    areas: HashMap<IdKey<crate::Area>, ()>,

    #[parse = "map/region.txt"]
    regions: TypeDefinition<crate::Region>,

    //#[parse = "map/superregion.txt"]
    //superregions: HashMap<IdKey<crate::Superregion>, Vec<IdRef<crate::Region>>>,

    #[parse = "map/continent.txt"]
    continents: HashMap<IdKey<crate::Continent>, Vec<u32>>,

    #[parse = "map/climate.txt"]
    climates: crate::ClimateList,

    #[parse = "common/cultures"]
    cultures: crate::CultureGroupList,

    #[parse = "common/religions"]
    religions: crate::ReligionList,

    #[parse = "common/ideas"]
    idea_groups: TypeDefinition<crate::IdeaGroup>,

    #[parse = "common/advisortypes"]
    advisors: TypeDefinition<crate::AdvisorType>,

    #[parse = "common/tradegoods"]
    tradegoods: TypeDefinition<crate::TradeGood>,

    #[parse = "common/tradenodes"]
    pub trade: TypeDefinition<crate::TradeNode>,

    //#[parse = "common/scripted_triggers"]
    //scripted_triggers: HashMap<IdKey<crate::ScriptedTrigger>, crate::ScriptedTrigger>,

    #[parse = "common/static_modifiers"]
    pub static_modifiers: HashMap<IdKey<crate::EventModifier>, crate::EventModifier>,

    #[parse = "common/event_modifiers"]
    pub event_modifiers: HashMap<IdKey<crate::EventModifier>, crate::EventModifier>,

    //#[parse = "events"]
    //events: crate::EventList,
}

macro_rules! impl_box {
    ($boxed_ty:ty, $id:literal) => { impl_box!($boxed_ty, $id, ""); };
    ($boxed_ty:ty, $id:literal, $default:literal) => {
        impl BoxedValue for $boxed_ty {
            const TYPE_VALUE: u32 = $id;
            const DEFAULT_STRING: &'static str = $default;
        }
    }
}

impl_box!(crate::Religion, 1, "noreligion");
impl_box!(crate::Country, 2, "---");
impl_box!(crate::Area, 3);
impl_box!(crate::Region, 4);
impl_box!(crate::Superregion, 5);
impl_box!(crate::Continent, 6);
impl_box!(crate::CultureGroup, 7);
impl_box!(crate::Culture, 8);
impl_box!(crate::TradeNode, 9);
impl_box!(crate::Climate, 10);
impl_box!(crate::TradeGood, 11);
impl_box!(crate::AdvisorType, 12);
impl_box!(crate::ScriptedTrigger, 13);
impl_box!(crate::IdeaGroup, 14);
impl_box!(crate::EventModifier, 15);
