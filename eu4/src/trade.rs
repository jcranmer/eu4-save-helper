use crate::{
    Modifiers,
    ProvinceCondition,
    ProvinceRef,
    RgbColor
};
use paradox::{ParadoxParse, ParserAtom};

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
    pub outgoing: Vec<TradeEdge>,
}

#[derive(ParadoxParse, Default, Debug)]
pub struct TradeEdge {
    pub name: ParserAtom, // XXX: tradenode ref
    path: Vec<ProvinceRef>,
    control: Vec<f64>
}

#[derive(ParadoxParse, Default, Debug)]
pub struct TradeGood {
    color: [paradox::FixedPoint; 3],
    #[optional] modifier: Modifiers,
    #[optional] province: Modifiers,
    #[optional] is_latent: bool,
    #[optional] is_valuable: bool,
    #[optional] rnw_latent_chance: u32,
    #[optional] trigger: (),
}
