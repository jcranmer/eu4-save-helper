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

pub fn parse_key_pair_list<S: FromParadoxKeyPair>(parser: &mut Parser
        ) -> Result<Vec<S>> {
    let mut vec = Vec::new();
    parser.parse_key_scope(|key, parser| {
        let value = parser.get_token()?.unwrap();
        vec.push(parser.try_parse(&key, value)?);
        Ok(())
    })?;
    Ok(vec)
}

fn parse_if<S: Condition>(parser: &mut Parser) -> Result<(Vec<S>, Vec<S>)> {
    let mut vec = Vec::new();
    let mut condition = None;
    parser.parse_key_scope(|key, parser| {
        if key.as_ref() == "limit" {
            condition = Some(parse_key_pair_list::<S>(parser)?);
        } else {
            let value = parser.get_token()?.unwrap();
            vec.push(parser.try_parse(&key, value)?);
        }
        Ok(())
    })?;
    Ok((condition.unwrap(), vec))
}

fn parse_extra<S: Condition, T: ParadoxParse + Default>(parser: &mut Parser,
        extra_key: &str) -> Result<(Vec<S>, T)> {
    let mut vec = Vec::new();
    let mut extra = T::default();
    parser.parse_key_scope(|key, parser| {
        if key.as_ref() == extra_key {
            extra.read(parser)?;
        } else {
            let value = parser.get_token()?.unwrap();
            vec.push(parser.try_parse(&key, value)?);
        }
        Ok(())
    })?;
    Ok((vec, extra))
}

impl <S: Condition> SpecialCondition<S> {
    pub fn try_parse(parser: &mut Parser, key: &str,
                     value: Token) -> Result<Option<Self>> {
        parser.unget(value);
        match key {
            "NOT" => Ok(Some(Self::Not(parse_key_pair_list(parser)?))),
            "AND" => Ok(Some(Self::And(parse_key_pair_list(parser)?))),
            "OR" => Ok(Some(Self::Or(parse_key_pair_list(parser)?))),
            "calc_true_if" => {
                let (conds, count) = parse_extra(parser, "amount")?;
                Ok(Some(Self::Many(conds, count)))
            }
            "hidden_trigger" =>
                Ok(Some(Self::Hidden(parse_key_pair_list(parser)?))),
            "if" => {
                let (condition, result) = parse_if(parser)?;
                Ok(Some(Self::If(condition, result)))
            },
            "else_if" => {
                let (condition, result) = parse_if(parser)?;
                Ok(Some(Self::ElseIf(condition, result)))
            },
            "else" => Ok(Some(Self::Else(parse_key_pair_list(parser)?))),
            _ => Ok(None)
        }
    }
}
