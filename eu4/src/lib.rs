type LocalizationKey = String; // XXX: LIES
type ProvinceRef = u32; // XXX: LIES
type RgbColor = Vec<u32>; // XXX: Lies

//include!(concat!(env!("OUT_DIR"), "/eu4_atom.rs"));
pub type Eu4Atom = paradox::ParserAtom;

#[macro_export]
macro_rules! eu4_atom {
    ($l:literal) => (crate::Eu4Atom::from($l))
}

mod advisors;
mod conditions;
mod country;
mod culture;
mod events;
mod misc;
mod modifiers;
mod game;
mod gamestate;
mod religion;
mod trade;

pub use advisors::*;
pub use conditions::*;
pub use country::*;
pub use culture::*;
pub use events::*;
pub use misc::*;
pub use modifiers::*;
pub use game::*;
pub use gamestate::Gamestate;
pub use religion::*;
pub use trade::*;
