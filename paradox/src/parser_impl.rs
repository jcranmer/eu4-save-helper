use crate::{BoxedValue, Date, FixedPoint, IdKey, IdRef};
use crate::parser::*;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::str::FromStr;

type DynError = Box<dyn StdError>;
type ParseResult = Result<(), ParseError>;

fn convert_err<T, E: StdError + 'static>(val: Result<T, E>)
        -> Result<T, DynError> {
    val.map_err(|err| err.into())
}

macro_rules! from_string {
    {$T:ty} => {
        impl ParadoxParse for $T {
            fn read_from(&mut self, _: &mut Parser, val: Token) -> ParseResult {
                let parsed = convert_err(<$T>::from_str(val.try_to_string()?))?;
                std::mem::replace(self, parsed);
                Ok(())
            }
        }
    };
    {$T:ty, $arm:ident} => {
        impl ParadoxParse for $T {
            fn read_from(&mut self, _: &mut Parser, val: Token) -> ParseResult {
                if let Token::$arm(val) = val {
                    *self = val;
                    return Ok(());
                }
                let parsed = convert_err(<$T>::from_str(val.try_to_string()?))?;
                std::mem::replace(self, parsed);
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

impl ParadoxParse for Date {
    fn read_from(&mut self, _: &mut Parser, val: Token) -> ParseResult {
        if let Token::Integer(val) = val {
            *self = crate::date::convert_date(val as u32);
            return Ok(());
        }
        let parsed = convert_err(val.try_to_string()?.parse())?;
        std::mem::replace(self, parsed);
        Ok(())
    }
}

macro_rules! impl_array {
    {$len:expr} => {
        impl <T: ParadoxParse> ParadoxParse for [T; $len] {
            fn read_from(&mut self, parser: &mut Parser,
                         val: Token) -> ParseResult {
                let class_name = std::any::type_name::<Self>();
                val.expect_complex()?;
                let mut i = 0;
                while let Some((key, v)) = parser.get_next_value()? {
                    if i == $len {
                        return parser.validation_error(class_name,
                            stringify!($len),
                            "too many entries in list", true, Some(v));
                    }
                    match key {
                        None => {
                            self[i].read_from(parser, v)?;
                            i += 1;
                        },
                        Some(key) => {
                            return parser.validation_error(class_name, &key,
                                "unexpected key in list", true, Some(v));
                        },
                    }
                }
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

impl ParadoxParse for bool {
    fn read_from(&mut self, _: &mut Parser, val: Token) -> ParseResult {
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

impl <T: ParadoxParse + Default> ParadoxParse for Vec<T> {
    fn read_from(&mut self, parser: &mut Parser, val: Token) -> ParseResult {
        val.expect_complex()?;
        while let Some((key, v)) = parser.get_next_value()? {
            match key {
                None => {
                    let mut parsed = T::default();
                    parsed.read_from(parser, v)?;
                    self.push(parsed);
                },
                Some(key) => {
                    return Err(ParseError::Constraint(
                            format!("Unexpected key {} in list", key)));
                },
            }
        }
        Ok(())
    }
}

impl <T: ParadoxParse + Default> ParadoxParse for HashMap<String, T> {
    fn read_from(&mut self, parser: &mut Parser, val: Token) -> ParseResult {
        val.expect_complex()?;
        while let Some((key, v)) = parser.get_next_value()? {
            match key {
                None => {
                    return Err(ParseError::Constraint(
                            format!("Unexpected keyless value in map")));
                },
                Some(key) => {
                    let key = key.into_owned();
                    let mut parsed = T::default();
                    parsed.read_from(parser, v)?;
                    if self.insert(key.clone(), parsed).is_some() {
                        return Err(ParseError::Constraint(
                            format!("Duplicate key {} in map", key)));
                    }
                }
            }
        }
        Ok(())
    }
}

impl <I, T: ParadoxParse + Default> ParadoxParse for HashMap<IdKey<I>, T>
    where I: BoxedValue, IdKey<I> : Eq + std::hash::Hash
{
    fn read_from(&mut self, parser: &mut Parser, val: Token) -> ParseResult {
        val.expect_complex()?;
        while let Some((key, v)) = parser.get_next_value()? {
            match key {
                None => {
                    return Err(ParseError::Constraint(
                            format!("Unexpected keyless value in map")));
                },
                Some(key) => {
                    let id = IdKey::new(
                        parser.get_game_data().get_id_box_mut::<I>(), &key);
                    let mut parsed = T::default();
                    parsed.read_from(parser, v)?;
                    if self.insert(id, parsed).is_some() {
                        return Err(ParseError::Constraint(
                            format!("Duplicate key {} in map", key)));
                    }
                }
            }
        }
        Ok(())
    }
}

impl <I, T: ParadoxParse + Default> ParadoxParse for HashMap<IdRef<I>, T>
    where I: BoxedValue, IdRef<I>: Default
{
    fn read_from(&mut self, parser: &mut Parser, val: Token) -> ParseResult {
        val.expect_complex()?;
        while let Some((key, v)) = parser.get_next_value()? {
            match key {
                None => {
                    return Err(ParseError::Constraint(
                            format!("Unexpected keyless value in map")));
                },
                Some(key) => {
                    let mut id: IdRef<I> = Default::default();
                    id.read_from(parser, Token::String(key.clone().into_owned()))?;
                    let mut parsed = T::default();
                    parsed.read_from(parser, v)?;
                    if self.insert(id, parsed).is_some() {
                        return Err(ParseError::Constraint(
                            format!("Duplicate key {} in map", key)));
                    }
                }
            }
        }
        Ok(())
    }
}

impl ParadoxParse for () {
    fn read_from(&mut self, parser: &mut Parser, val: Token) -> ParseResult {
        match val {
            Token::LBrace => {
                while let Some((_, v)) = parser.get_next_value()? {
                    let mut parsed = ();
                    parsed.read_from(parser, v)?;
                }
            },
            _ => ()
        }
        Ok(())
    }
}
