use crate::{CountryCondition, CountryModifier, Weight};
use paradox::{ParadoxParse, ParseError, Parser};
use std::collections::HashMap;

#[derive(ParadoxParse, Default)]
pub struct IdeaGroup {
    #[optional] pub start: Vec<CountryModifier>,
    pub bonus: Vec<CountryModifier>,
    #[optional] pub trigger: (),
    #[optional] pub free: bool,
    #[optional] pub ai_will_do: Weight,
    #[optional] pub important: bool,
    #[optional] pub category: String,

    #[collect] pub ideas: HashMap<String, Idea>
}

#[derive(Default)]
pub struct Idea(Vec<CountryModifier>);

impl ParadoxParse for Idea {
    fn read(&mut self, parser: &mut Parser) -> Result<(), ParseError> {
        self.0 = paradox::parse_key_pair_list(parser)?;
        Ok(())
    }
}
