use paradox::ParadoxParse;

type ProvinceRef = u32; // XXX: LIES
type RgbColor = Vec<u32>; // XXX: Lies

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

pub type TradeNodeList = std::collections::HashMap<String, TradeNode>;
