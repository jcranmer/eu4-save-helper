use crate::CountryCondition;
use paradox::ParadoxParse;

#[derive(ParadoxParse, Default)]
pub struct CountryEvent {
    pub id: String,
    pub title: String,
    //pub desc: String,
    //pub picture: String,
    #[optional] pub hidden: bool,
    #[optional] pub fire_only_once: bool,
    #[optional] pub is_triggered_only: bool,
    #[optional] pub major: bool,

    pub trigger: Vec<CountryCondition>
}

#[derive(ParadoxParse, Default)]
pub struct EventList {
    #[repeated]
    pub namespace: Vec<String>,
    #[optional] pub normal_or_historical_nations: bool,
    #[repeated]
    pub country_event: Vec<CountryEvent>
}
