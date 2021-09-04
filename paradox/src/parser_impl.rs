use crate::{Date, FixedPoint, GameTrait, Token};
use crate::parser::*;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::str::FromStr;
use string_cache::{Atom, StaticAtomSet};

type DynError = Box<dyn StdError>;
type ParseResult = Result<(), ParseError>;

fn convert_err<T, E: StdError + 'static>(val: Result<T, E>)
        -> Result<T, DynError> {
    val.map_err(|err| err.into())
}

macro_rules! from_string {
    {$T:ty} => {
        impl <G: GameTrait> ParadoxParse<G> for $T {
            fn read(&mut self, parser: &mut Parser<G>) -> ParseResult {
                let val = parser.get_token()?.ok_or(ParseError::Eof)?;
                *self = convert_err(<$T>::from_str(val.try_to_string()?))?;
                Ok(())
            }
        }
    };
    {$T:ty, $arm:ident} => {
        impl <G: GameTrait> ParadoxParse<G> for $T {
            fn read(&mut self, parser: &mut Parser<G>) -> ParseResult {
                let val = parser.get_token()?.ok_or(ParseError::Eof)?;
                if let Token::$arm(val) = val {
                    *self = val;
                    return Ok(());
                }
                *self = convert_err(<$T>::from_str(val.try_to_string()?))?;
                Ok(())
            }
        }
    }
}

from_string!{i32, Integer}
from_string!{u32, Unsigned}
from_string!{f32}
from_string!{f64, Float}
from_string!{String}
from_string!{FixedPoint, Fixed}

impl <G: GameTrait> ParadoxParse<G> for Date {
    fn read(&mut self, parser: &mut Parser<G>) -> ParseResult {
        let val = parser.get_token()?.ok_or(ParseError::Eof)?;
        if let Token::Integer(val) = val {
            *self = crate::date::convert_date(val as u32);
            return Ok(());
        }
        *self = convert_err(val.try_to_string()?.parse())?;
        Ok(())
    }
}

impl <G: GameTrait, Static: StaticAtomSet> ParadoxParse<G> for Atom<Static> {
    fn read(&mut self, parser: &mut Parser<G>) -> ParseResult {
        let val = parser.get_token()?.ok_or(ParseError::Eof)?;
        *self = Self::from(val);
        Ok(())
    }
}

macro_rules! impl_array {
    {$len:expr} => {
        impl <G: GameTrait, T: ParadoxParse<G>> ParadoxParse<G> for [T; $len] {
            fn read(&mut self, parser: &mut Parser<G>) -> ParseResult {
                let mut i = 0;
                let class_name = std::any::type_name::<Self>();
                parser.with_scope(|parser| {
                    if i == $len {
                        return parser.validation_error(class_name,
                            stringify!($len),
                            "too many entries in list", true, None);
                    }
                    self[i].read(parser)?;
                    i += 1;
                    Ok(())
                })?;
                if i != $len {
                    return parser.validation_error(class_name, &i.to_string(),
                        "list terminated early", true, None);
                }
                Ok(())
            }
        }
    }
}

impl_array!{1}
impl_array!{2}
impl_array!{3}
impl_array!{4}
impl_array!{5}
impl_array!{6}
impl_array!{7}
impl_array!{8}
impl_array!{9}
impl_array!{10}
impl_array!{11}
impl_array!{12}
impl_array!{13}
impl_array!{14}
impl_array!{15}
impl_array!{16}
impl_array!{17}
impl_array!{18}
impl_array!{19}
impl_array!{20}
impl_array!{21}
impl_array!{22}
impl_array!{23}
impl_array!{24}
impl_array!{25}
impl_array!{26}
impl_array!{27}
impl_array!{28}
impl_array!{29}
impl_array!{30}
impl_array!{31}
impl_array!{32}

impl <G: GameTrait> ParadoxParse<G> for bool {
    fn read(&mut self, parser: &mut Parser<G>) -> ParseResult {
        let val = parser.get_token()?.ok_or(ParseError::Eof)?;
        if let Token::Bool(b) = val {
            *self = b;
            return Ok(());
        }
        let string = val.try_to_string()?;
        if string == "yes" {
            *self = true;
        } else if string == "no" {
            *self = false;
        } else {
            return Err(ParseError::Constraint(
                format!("Expected bool string, found {}", string)));
        }
        Ok(())
    }
}

impl <G: GameTrait, T: ParadoxParse<G> + Default> ParadoxParse<G> for Vec<T> {
    fn read(&mut self, parser: &mut Parser<G>) -> ParseResult {
        parser.with_scope(|parser| {
            let mut value = T::default();
            value.read(parser)?;
            self.push(value);
            Ok(())
        })
    }
}

impl <G: GameTrait, T: ParadoxParse<G> + Default> ParadoxParse<G> for HashMap<String, T> {
    fn read(&mut self, parser: &mut Parser<G>) -> ParseResult {
        parser.parse_key_scope(|key, parser| {
            let mut val = T::default();
            val.read(parser)?;
            if self.insert(format!("{}", key), val).is_some() {
                return Err(ParseError::Constraint(
                        format!("Duplicate key {} in map", key)));
            }
            Ok(())
        })
    }
}

impl <G: GameTrait, T: ParadoxParse<G> + Default> ParadoxParse<G> for HashMap<ParserAtom, T> {
    fn read(&mut self, parser: &mut Parser<G>) -> ParseResult {
        parser.parse_key_scope(|key, parser| {
            let mut val = T::default();
            val.read(parser)?;
            if self.insert(key.clone(), val).is_some() {
                // Some maps have duplicate keys!
                //return Err(ParseError::Constraint(
                //        format!("Duplicate key {} in map", key)));
            }
            Ok(())
        })
    }
}

impl <G: GameTrait> ParadoxParse<G> for () {
    fn read(&mut self, parser: &mut Parser<G>) -> ParseResult {
        match parser.get_token()? {
            None => Err(ParseError::Eof),
            Some(Token::LBrace) => {
                parser.unget(Token::LBrace);
                parser.with_scope(|parser| {
                    ().read(parser)
                })
            },
            Some(Token::RBrace) => Err(ParseError::Parse(Token::RBrace)),
            Some(_) => Ok(()),
        }
    }
}
