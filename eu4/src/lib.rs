type LocalizationKey = String; // XXX: LIES
type ProvinceRef = u32; // XXX: LIES
type RgbColor = Vec<u32>; // XXX: Lies

mod effects;
mod gamestate;
mod religion;
mod trade;

pub use effects::*;
pub use gamestate::*;
pub use religion::*;
pub use trade::*;
