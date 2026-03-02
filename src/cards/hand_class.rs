use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandClass {
    HighCard,
    Pair,
    TwoPair,
    Trips,
    Straight,
    Flush,
    FullHouse,
    Quads,
    StraightFlush,
}

impl HandClass {
    pub const ALL: [HandClass; 9] = [
        HandClass::HighCard,
        HandClass::Pair,
        HandClass::TwoPair,
        HandClass::Trips,
        HandClass::Straight,
        HandClass::Flush,
        HandClass::FullHouse,
        HandClass::Quads,
        HandClass::StraightFlush,
    ];

    pub const fn strength(self) -> u8 {
        match self {
            HandClass::HighCard => 0,
            HandClass::Pair => 1,
            HandClass::TwoPair => 2,
            HandClass::Trips => 3,
            HandClass::Straight => 4,
            HandClass::Flush => 5,
            HandClass::FullHouse => 6,
            HandClass::Quads => 7,
            HandClass::StraightFlush => 8,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            HandClass::HighCard => "high-card",
            HandClass::Pair => "pair",
            HandClass::TwoPair => "two-pair",
            HandClass::Trips => "trips",
            HandClass::Straight => "straight",
            HandClass::Flush => "flush",
            HandClass::FullHouse => "full-house",
            HandClass::Quads => "quads",
            HandClass::StraightFlush => "straight-flush",
        }
    }
}

impl fmt::Display for HandClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
