use crate::{
    Eu4Atom,
    Eu4Trait,
    Modifiers,
    ProvinceRef,
    RgbColor
};
use paradox::{FixedPoint, ParadoxParse, ParseError, Parser};

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
    pub name: Eu4Atom, // XXX: tradenode ref
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

#[derive(ParadoxParse, Default)]
pub struct TradePolicy {
    #[optional] can_select: (),
    #[optional] can_maintain: (),
    button_gfx: String,
    #[optional] center_of_reformation: bool,
    #[optional] unique: bool,
    #[optional] show_alert: bool,
    #[optional] countries_with_merchant_modifier: Modifiers,
    #[optional] node_province_modifier: Modifiers,
    #[optional] trade_power: ConfusingThing
}

impl TradePolicy {
    pub fn get_trade_power_modifier(&self) -> FixedPoint {
        self.trade_power.modifier
    }
}

#[derive(Default)]
struct ConfusingThing {
    modifier: FixedPoint
}

impl ParadoxParse<Eu4Trait> for ConfusingThing {
    fn read(&mut self, parser: &mut Parser<Eu4Trait>) -> Result<(), ParseError> {
        parser.parse_key_scope(|key, parser| {
            match key {
                eu4_atom!("power_modifier") => self.modifier.read(parser),
                _ => ().read(parser),
            }
        })
    }
}
