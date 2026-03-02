use core::fmt;

use crate::cards::{Rank, Suit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn from_shorthand(value: &str) -> Option<Self> {
        let mut chars = value.chars();
        let rank = Rank::from_short(chars.next()?)?;
        let suit = Suit::from_short(chars.next()?)?;

        if chars.next().is_some() {
            return None;
        }

        Some(Self::new(rank, suit))
    }

    pub fn shorthand(self) -> String {
        format!("{}{}", self.rank.short(), self.suit.short())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
