#![doc = include_str!("../README.md")]

pub mod cells;
mod constants;
pub mod utils;

mod types;
pub use crate::types::Cell;

#[cfg(test)]
mod test;
