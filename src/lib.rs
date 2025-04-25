#![doc = include_str!("../README.md")]

pub mod cells;
mod constants;
pub mod error;
pub mod utils;

mod types;
pub use crate::types::Cell;

mod direction;
pub use crate::direction::Direction;

#[cfg(test)]
mod test;
