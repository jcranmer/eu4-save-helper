use paradox_derive::ParadoxParse;

type ProvinceRef = u32; // XXX: LIES
type RgbColor = Vec<u32>; // XXX: Lies

#[derive(ParadoxParse, Default, Debug)]
pub struct TradeNode {
    location: ProvinceRef,
    inland: bool,
    end: bool,
    ai_will_propagate_through_trade: bool,
    members: Vec<ProvinceRef>,
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::prelude::*;
    use std::fs::File;
    use paradox::*;
    use super::*;

    #[test]
    fn load_trade() -> Result<(), ParseError> {
        let file = File::open("/home/jcranmer/.steam/steam/SteamApps/common/Europa Universalis IV/common/tradenodes/00_tradenodes.txt")?;
        let lexer = TextLexer::new(Box::new(file), "00_tradenodes.txt".into());
        let mut trade_nodes : HashMap<String, TradeNode> = Default::default();
        Parser::new(Box::new(lexer)).parse(&mut trade_nodes)?;
        for (key, tradenode) in trade_nodes {
            for next in tradenode.outgoing {
                println!("{} -> {}", key, next.name);
            }
        }
        assert_eq!(1, 2, "oops");
        Ok(())
    }
}
