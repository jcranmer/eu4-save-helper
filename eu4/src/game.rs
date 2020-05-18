use paradox::BoxedValue;
use std::collections::HashMap;

#[derive(paradox::GameData)]
pub struct GameData {
    pub base_info: paradox::GameData,

    #[parse = "common/religions"]
    religions: crate::ReligionList,

    #[parse = "common/tradenodes"]
    trade: crate::TradeNodeList,

    #[parse = "map/area.txt"]
    areas: HashMap<String, ()>,
}

macro_rules! impl_box {
    ($boxed_ty:ty, $id:literal) => {
        impl BoxedValue for $boxed_ty {
            const TYPE_VALUE: u32 = $id;
        }
    }
}

impl_box!(crate::Religion, 1);
