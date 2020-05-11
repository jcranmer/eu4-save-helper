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

