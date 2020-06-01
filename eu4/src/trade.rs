use crate::{
    CountryModifier,
    ProvinceCondition,
    ProvinceModifier,
    ProvinceRef,
    RgbColor
};
use paradox::{IdKey, ParadoxParse};

#[derive(ParadoxParse, Default, Debug)]
pub struct TradeNode {
    location: ProvinceRef,
    #[optional]
    inland: bool,
    #[optional]
    end: bool,
    #[optional]
    ai_will_propagate_through_trade: bool,
    members: Vec<ProvinceRef>,
    #[optional]
    color: RgbColor,
    #[repeated]
    outgoing: Vec<TradeEdge>,
}

#[derive(ParadoxParse, Default, Debug)]
pub struct TradeEdge {
    name: String, // XXX: tradenode ref
    path: Vec<ProvinceRef>,
    control: Vec<f64>
}

#[derive(ParadoxParse, Default, Debug)]
pub struct TradeGood {
    color: [paradox::FixedPoint; 3],
    #[optional] modifier: Vec<CountryModifier>,
    #[optional] province: Vec<ProvinceModifier>,
    #[optional] is_latent: bool,
    #[optional] is_valuable: bool,
    #[optional] rnw_latent_chance: u32,
    #[optional] trigger: Vec<ProvinceCondition>,
}

pub type TradeGoodList = std::collections::HashMap<IdKey<TradeGood>, TradeGood>;
pub type TradeNodeList = std::collections::HashMap<IdKey<TradeNode>, TradeNode>;
