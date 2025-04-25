use crate::error;
use core::fmt;

/// Maximum value for a direction.
const MAX: u8 = 4;

/// A direction within an rectangular grid.
///
/// In Quadbin, each cell at level `N-1` is divided into 4 cells at the
/// level `N`, with each sub-cell in one of the 4 possible directions (4 axes).
///
// TODO:
// Add schema of direction as in h3o
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(u8)]
pub enum Direction {
    /// North.
    Up = 0,
    /// East.
    Right = 1,
    /// West.
    Left = 2,
    /// South.
    Down = 3,
} // TODO: Add Center as in h3o?

// Updated after h3o crate
// https://github.com/HydroniumLabs/h3o/blob/master/src/direction.rs
impl Direction {
    /// Iterates over the valid directions.
    ///
    /// # Example
    ///
    /// ```
    /// use quadbin::Direction;
    ///
    /// let directions = Direction::iter().collect::<Vec<_>>();
    /// ```
    pub fn iter() -> impl Iterator<Item = Self> {
        // SAFETY: values from 0 to MAX are valid directions.
        (0..=MAX).map(Self::new_unchecked)
    }

    /// Initializes a new [`Direction`] using a value that may be out of range.
    ///
    /// # Safety
    ///
    /// The value must be a valid direction.
    #[expect(unsafe_code, reason = "only used internally")]
    pub(crate) const fn new_unchecked(value: u8) -> Self {
        assert!(value <= MAX, "direction out of range");
        // SAFETY: range checked above.
        unsafe { core::mem::transmute::<u8, Self>(value) }
    }
}

impl TryFrom<u8> for Direction {
    type Error = error::InvalidDirection;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Up),
            1 => Ok(Self::Right),
            2 => Ok(Self::Left),
            3 => Ok(Self::Down),
            _ => Err(Self::Error::new(value, "out of range")),
        }
    }
}

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        value as Self
    }
}

impl From<Direction> for u64 {
    fn from(value: Direction) -> Self {
        u8::from(value).into()
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        u8::from(value).into()
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}
