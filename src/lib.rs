#![doc = include_str!("../README.md")]

// Quadbin cell itself
mod cells;
mod geo;
pub use crate::cells::Cell;

// Direction struct
mod directions;
pub use crate::directions::Direction;

// Errors
pub mod errors;

// Internal stuff
mod constants;
mod tiles;
mod utils;

#[cfg(test)]
mod test;
