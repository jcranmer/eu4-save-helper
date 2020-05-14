type LocalizationKey = String; // XXX: LIES
type ProvinceRef = u32; // XXX: LIES
type RgbColor = Vec<u32>; // XXX: Lies

mod religion;
mod trade;

pub use religion::*;
pub use trade::*;
