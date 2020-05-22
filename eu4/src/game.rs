use paradox::{BoxedValue, IdKey, IdRef};
use std::collections::HashMap;

#[derive(paradox::GameData)]
pub struct GameData {
    pub base_info: paradox::GameData,

    #[parse = "common/country_tags"]
    tags: crate::CountryMap,

    #[parse = "map/area.txt"]
    areas: HashMap<IdKey<crate::Area>, ()>,

    #[parse = "map/region.txt"]
    regions: HashMap<IdKey<crate::Region>, crate::Region>,

    #[parse = "map/superregion.txt"]
    superregions: HashMap<IdKey<crate::Superregion>, Vec<IdRef<crate::Region>>>,

    #[parse = "map/continent.txt"]
    continents: HashMap<IdKey<crate::Continent>, Vec<u32>>,

    #[parse = "common/cultures"]
    cultures: crate::CultureGroupList,

    #[parse = "common/religions"]
    religions: crate::ReligionList,

    #[parse = "common/tradenodes"]
    trade: crate::TradeNodeList,

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
