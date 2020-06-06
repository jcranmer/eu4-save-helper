use crate::{FromParadoxKeyPair, ParadoxParse, Parser, ParseError, Token};
type Result<T> = std::result::Result<T, ParseError>;

pub trait Condition : FromParadoxKeyPair {
}

pub trait Scope { }

#[derive(Debug)]
pub enum SpecialCondition<S: Condition> {
    Not(Vec<S>),
    And(Vec<S>),
    Or(Vec<S>),
    Many(Vec<S>, u32),
    Hidden(Vec<S>),
    If(Vec<S>, Vec<S>),
    ElseIf(Vec<S>, Vec<S>),
    Else(Vec<S>)
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

fn parse_if<S: Condition>(parser: &mut Parser,
                          value: Token) -> Result<(Vec<S>, Vec<S>)> {
    let class_name = std::any::type_name::<Vec<S>>();
    let mut vec = Vec::new();
    value.expect_complex()?;
    let condition = match parser.get_next_value()? {
        Some((Some(key), value)) if key == "limit" => {
            parse_key_pair_list::<S>(parser, value)?
        },
        _ => {
            parser.validation_error("if", "", "missing limit", true, None)?;
            Vec::new()
        }
    };
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
    Ok((condition, vec))
}

fn parse_extra<S: Condition, T: ParadoxParse + Default>(parser: &mut Parser,
        extra_key: &str, value: Token) -> Result<(Vec<S>, T)> {
    let class_name = std::any::type_name::<Vec<S>>();
    let mut vec = Vec::new();
    let mut extra = T::default();
    value.expect_complex()?;
    while let Some((key, value)) = parser.get_next_value()? {
        match key {
            None => {
                parser.validation_error(class_name, "", "bad_key", false,
                                        Some(value))?;
            },
            Some(key) if key == extra_key => {
                extra.read_from(parser, value)?;
            },
            Some(key) => {
                let key = key.into_owned();
                vec.push(parser.try_parse(&key, value)?);
            },
        }
    }
    Ok((vec, extra))
}

impl <S: Condition> SpecialCondition<S> {
    pub fn try_parse(parser: &mut Parser, key: &str,
                     value: Token) -> Result<Option<Self>> {
        match key {
            "NOT" => Ok(Some(Self::Not(parse_key_pair_list(parser, value)?))),
            "AND" => Ok(Some(Self::And(parse_key_pair_list(parser, value)?))),
            "OR" => Ok(Some(Self::Or(parse_key_pair_list(parser, value)?))),
            "calc_true_if" => {
                let (conds, count) = parse_extra(parser, "amount", value)?;
                Ok(Some(Self::Many(conds, count)))
            }
            "hidden_trigger" =>
                Ok(Some(Self::Hidden(parse_key_pair_list(parser, value)?))),
            "if" => {
                let (condition, result) = parse_if(parser, value)?;
                Ok(Some(Self::If(condition, result)))
            },
            "else_if" => {
                let (condition, result) = parse_if(parser, value)?;
                Ok(Some(Self::ElseIf(condition, result)))
            },
            "else" => Ok(Some(Self::Else(parse_key_pair_list(parser, value)?))),
            _ => Ok(None)
        }
    }
}
