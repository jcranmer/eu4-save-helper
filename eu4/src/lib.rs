type LocalizationKey = String; // XXX: LIES
type ProvinceRef = u32; // XXX: LIES
type RgbColor = Vec<u32>; // XXX: Lies

mod advisors;
mod conditions;
mod country;
mod culture;
mod effects;
mod events;
mod modifiers;
mod game;
mod gamestate;
mod religion;
mod scope;
mod trade;

pub use advisors::*;
pub use conditions::*;
pub use country::*;
pub use culture::*;
pub use effects::*;
pub use events::*;
pub use modifiers::*;
pub use game::*;
pub use gamestate::Gamestate;
pub use religion::*;
pub use scope::*;
pub use trade::*;
