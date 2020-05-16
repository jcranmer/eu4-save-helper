use crate::parser::*;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::str::FromStr;

type DynError = Box<dyn StdError>;

fn convert_err<T, E: StdError + 'static>(val: Result<T, E>)
        -> Result<T, DynError> {
    val.map_err(|err| err.into())
}

macro_rules! from_string {
    {$T:ty} => {
        impl ParadoxParse for $T {
            fn read_from(&mut self,
                         val: UnparsedValue<'_>) -> Result<(), ParseError> {
                let parsed = convert_err(<$T>::from_str(&val.into_string()?))?;
                std::mem::replace(self, parsed);
                Ok(())
            }
        }
    }
}

from_string!{i32}
from_string!{u32}
from_string!{f32}
from_string!{f64}
from_string!{String}
from_string!{crate::Date}
from_string!{crate::FixedPoint}

macro_rules! impl_array {
    {$len:expr} => {
        impl <T: ParadoxParse> ParadoxParse for [T; $len] {
            fn read_from(&mut self,
                         mut val: UnparsedValue<'_>) -> Result<(), ParseError> {
                let mut i = 0;
                loop {
                    let next_pair = val.next_key_value_pair()?;
                    match next_pair {
                        None => {
                            return if i != $len {
                                val.validation_error(stringify!([T; $len]), "",
                                    &format!("Expected {} entries, found {}",
                                            $len, i), true)
                            } else {
                                Ok(())
                            }
                        }
                        Some((None, v)) => {
                            if i == $len {
                                v.validation_error(stringify!([T; $len]), "",
                                    "too many entries in list", true)?;
                            }
                            self[i].read_from(v)?;
                            i += 1;
                        },
                        Some((Some(key), _)) => {
                            return Err(ParseError::Constraint(
                                    format!("Unexpected key {} in list", key)));
                        },
                    }
                }
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
    fn read_from(&mut self, val: UnparsedValue) -> Result<(), ParseError> {
        let string = &val.into_string()?;
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
    fn read_from(&mut self,
                 mut val: UnparsedValue<'_>) -> Result<(), ParseError> {
        loop {
            let next_pair = val.next_key_value_pair()?;
            match next_pair {
                None => return Ok(()),
                Some((None, v)) => {
                    let mut parsed = T::default();
                    parsed.read_from(v)?;
                    self.push(parsed);
                },
                Some((Some(key), _)) => {
                    return Err(ParseError::Constraint(
                            format!("Unexpected key {} in list", key)));
                },
            }
        }
    }
}

impl <T: ParadoxParse + Default> ParadoxParse for HashMap<String, T> {
    fn read_from(&mut self,
                 mut val: UnparsedValue<'_>) -> Result<(), ParseError> {
        loop {
            let next_pair = val.next_key_value_pair()?;
            match next_pair {
                None => return Ok(()),
                Some((None, _)) => {
                    return Err(ParseError::Constraint(
                            format!("Unexpected keyless value in map")));
                },
                Some((Some(key), v)) => {
                    let mut parsed = T::default();
                    parsed.read_from(v)?;
                    if self.insert(key.clone(), parsed).is_some() {
                        return Err(ParseError::Constraint(
                            format!("Duplicate key {} in map", key)));
                    }
                },
            }
        }
    }
}

impl ParadoxParse for () {
    fn read_from(&mut self,
                 mut val: UnparsedValue<'_>) -> Result<(), ParseError> {
        match val {
            UnparsedValue::Simple(_) => Ok(()),
            UnparsedValue::Complex { parser: _, level: _ } => {
                loop {
                    let next_pair = val.next_key_value_pair()?;
                    match next_pair {
                        None => return Ok(()),
                        Some((_, v)) => {
                            let mut parsed = ();
                            parsed.read_from(v)?;
                        },
                    }
                }
            }
        }
    }
}
