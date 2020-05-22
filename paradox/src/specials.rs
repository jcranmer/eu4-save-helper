use crate::{FromParadoxKeyPair, Parser, ParseError, Token};
type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub enum SpecialCondition<S> {
    Not(Vec<S>),
    And(Vec<S>),
    Or(Vec<S>),
}

pub fn parse_key_pair_list<S: FromParadoxKeyPair>(parser: &mut Parser,
        value: Token) -> Result<Vec<S>> {
    let class_name = std::any::type_name::<Vec<S>>();
    let mut vec = Vec::new();
    value.expect_complex()?;
    while let Some((key, value)) = parser.get_next_value()? {
        match key {
            None => {
                parser.validation_error(class_name, "", "bad_key", false,
                                        Some(value))?;
            },
            Some(key) => {
                let key = key.into_owned();
                vec.push(parser.try_parse(&key, value)?);
            },
        }
    }
    Ok(vec)
}

impl <S: FromParadoxKeyPair> SpecialCondition<S> {
    pub fn try_parse(parser: &mut Parser, key: &str,
                     value: Token) -> Result<Option<Self>> {
        match key {
            "NOT" => Ok(Some(Self::Not(parse_key_pair_list(parser, value)?))),
            "AND" => Ok(Some(Self::And(parse_key_pair_list(parser, value)?))),
            "OR" => Ok(Some(Self::Or(parse_key_pair_list(parser, value)?))),
            _ => Ok(None)
        }
    }
}
