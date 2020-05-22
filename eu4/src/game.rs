use paradox::BoxedValue;
use std::collections::HashMap;

#[derive(paradox::GameData)]
pub struct GameData {
    pub base_info: paradox::GameData,

    #[parse = "common/country_tags"]
    tags: crate::CountryMap,

    #[parse = "common/religions"]
    religions: crate::ReligionList,

    #[parse = "common/tradenodes"]
    trade: crate::TradeNodeList,

    #[parse = "map/area.txt"]
    areas: HashMap<String, ()>,

    //#[parse = "events"]
    //events: crate::EventList,
}

macro_rules! impl_box {
    ($boxed_ty:ty, $id:literal, $default:literal) => {
        impl BoxedValue for $boxed_ty {
            const TYPE_VALUE: u32 = $id;
            const DEFAULT_STRING: &'static str = $default;
        }
    }
}

impl_box!(crate::Religion, 1, "noreligion");
impl_box!(crate::Country, 2, "---");
