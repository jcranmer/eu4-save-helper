use crate::Factor;
use paradox::{ParadoxParse};

#[derive(ParadoxParse, Default)]
pub struct CountryEvent {
    pub id: String,
    pub title: String,
    #[repeated] pub desc: Vec<()>, // XXX: These are rather more complicated...
    #[repeated] pub picture: Vec<()>, // XXX: These are rather more complicated...
    #[optional] pub hidden: bool,
    #[optional] pub fire_only_once: bool,
    #[optional] pub is_triggered_only: bool,
    #[optional] pub major: bool,

    #[optional] pub mean_time_to_happen: MeanTimeToHappen,

    pub trigger: (), // Vec<CountryCondition>,
}

#[derive(ParadoxParse, Default)]
pub struct EventList {
    #[repeated]
    pub namespace: Vec<String>,
    #[optional] pub normal_or_historical_nations: bool,
    #[repeated]
    pub country_event: Vec<CountryEvent>
}

#[derive(ParadoxParse, Default)]
pub struct MeanTimeToHappen {
    #[optional] pub days: u32,
    #[optional] pub months: u32,
    #[repeated] pub modifier: Vec<Factor>,
}
