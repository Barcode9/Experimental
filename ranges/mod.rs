//! Utilities for working with one-dimensional and two-dimensional ranges.

pub mod combo;
pub mod grid;
pub mod weighted_range;

pub use combo::RangeCombo;
pub use grid::{GridPoint, GridRange};
pub use weighted_range::WeightedRange;
