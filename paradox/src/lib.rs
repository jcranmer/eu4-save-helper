//! This crate provides an implementation of core concepts within Paradox's
//! Clausewitz Engine (used in Europa Universalis, Crusader Kings, and Stellaris
//! among other games).
//!
//! The utilities provided are:
//! * A representation of the date system used in the games.
//! * An implementation of the fixed-point arithmetic they use.
//! * A parser for their internal format.

// Set up #[derive(ParadoxParse)] support.
#[allow(unused_imports)]
#[macro_use]
extern crate paradox_derive;
pub use paradox_derive::*;

mod date;
mod parser;
mod parser_impl;

pub use date::*;
pub use parser::*;
