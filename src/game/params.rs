//! Static parameters for a two-player push/fold game.

/// Immutable game configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameParams {
    /// Effective stack size for both players.
    pub stack: u32,
    /// Small blind posted by the button.
    pub small_blind: u32,
    /// Big blind posted by the defender.
    pub big_blind: u32,
}

impl GameParams {
    /// Creates validated game parameters.
    ///
    /// Returns `None` when blind levels are invalid or the stack is too small.
    pub fn new(stack: u32, small_blind: u32, big_blind: u32) -> Option<Self> {
        if small_blind == 0 || big_blind == 0 || small_blind > big_blind {
            return None;
        }

        if stack <= big_blind {
            return None;
        }

        Some(Self {
            stack,
            small_blind,
            big_blind,
        })
    }

    /// Chips in the pot before any action (blinds posted).
    pub const fn initial_pot(self) -> u32 {
        self.small_blind + self.big_blind
    }
}

impl Default for GameParams {
    fn default() -> Self {
        Self {
            stack: 20,
            small_blind: 1,
            big_blind: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameParams;

    #[test]
    fn validates_blinds_and_stack() {
        assert!(GameParams::new(20, 1, 2).is_some());
        assert!(GameParams::new(20, 0, 2).is_none());
        assert!(GameParams::new(20, 3, 2).is_none());
        assert!(GameParams::new(2, 1, 2).is_none());
    }
}
