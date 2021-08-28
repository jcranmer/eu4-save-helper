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
            fn read(&mut self, parser: &mut Parser) -> ParseResult {
                let val = parser.get_token()?.ok_or(ParseError::Eof)?;
                *self = convert_err(<$T>::from_str(val.try_to_string()?))?;
                Ok(())
            }
        }
    };
    {$T:ty, $arm:ident} => {
        impl ParadoxParse for $T {
            fn read(&mut self, parser: &mut Parser) -> ParseResult {
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

impl ParadoxParse for Date {
    fn read(&mut self, parser: &mut Parser) -> ParseResult {
        let val = parser.get_token()?.ok_or(ParseError::Eof)?;
        if let Token::Integer(val) = val {
            *self = crate::date::convert_date(val as u32);
            return Ok(());
        }
        *self = convert_err(val.try_to_string()?.parse())?;
        Ok(())
    }
}

impl ParadoxParse for ParserAtom {
    fn read(&mut self, parser: &mut Parser) -> ParseResult {
        let val = parser.get_token()?.ok_or(ParseError::Eof)?;
        *self = ParserAtom::from(val);
        Ok(())
    }
}

macro_rules! impl_array {
    {$len:expr} => {
        impl <T: ParadoxParse> ParadoxParse for [T; $len] {
            fn read(&mut self, parser: &mut Parser) -> ParseResult {
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

impl ParadoxParse for bool {
    fn read(&mut self, parser: &mut Parser) -> ParseResult {
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

impl <T: ParadoxParse + Default> ParadoxParse for Vec<T> {
    fn read(&mut self, parser: &mut Parser) -> ParseResult {
        parser.with_scope(|parser| {
            let mut value = T::default();
            value.read(parser)?;
            self.push(value);
            Ok(())
        })
    }
}

impl <T: ParadoxParse + Default> ParadoxParse for Vec<(ParserAtom, T)> {
    fn read(&mut self, parser: &mut Parser) -> ParseResult {
        parser.parse_key_scope(|key, parser| {
            let mut value = T::default();
            value.read(parser)?;
            self.push((key, value));
            Ok(())
        })
    }
}

impl <T: ParadoxParse + Default> ParadoxParse for HashMap<String, T> {
    fn read(&mut self, parser: &mut Parser) -> ParseResult {
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

impl <T: ParadoxParse + Default> ParadoxParse for HashMap<ParserAtom, T> {
    fn read(&mut self, parser: &mut Parser) -> ParseResult {
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

impl <I, T: ParadoxParse + Default> ParadoxParse for HashMap<IdKey<I>, T>
    where I: BoxedValue, IdKey<I> : Eq + std::hash::Hash
{
    fn read(&mut self, parser: &mut Parser) -> ParseResult {
        parser.parse_key_scope(|key, parser| {
            let mut val = T::default();
            val.read(parser)?;
            let id = IdKey::new(
                parser.get_game_data().get_id_box_mut::<I>(), &key);
            if self.insert(id, val).is_some() {
                // Some maps have duplicate keys!
                //return Err(ParseError::Constraint(
                //    format!("Duplicate key {} in map", key)));
            }
            Ok(())
        })
    }
}

impl <I, T: ParadoxParse + Default> ParadoxParse for HashMap<IdRef<I>, T>
    where I: BoxedValue, IdRef<I>: Default
{
    fn read(&mut self, parser: &mut Parser) -> ParseResult {
        parser.parse_key_scope(|key, parser| {
            let mut id: IdRef<I> = Default::default();
            parser.unget(Token::String(key.as_ref().into()));
            id.read(parser)?;
            let mut val = T::default();
            val.read(parser)?;
            if self.insert(id, val).is_some() {
                return Err(ParseError::Constraint(
                        format!("Duplicate key {} in map", key)));
            }
            Ok(())
        })
    }
}

impl ParadoxParse for () {
    fn read(&mut self, parser: &mut Parser) -> ParseResult {
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
