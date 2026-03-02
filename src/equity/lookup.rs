//! Lookup primitives for storing aggregate equity outcomes.

/// Aggregated showdown outcomes for a hero hand/range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EquityLookup {
    wins: u64,
    ties: u64,
    losses: u64,
}

impl EquityLookup {
    /// Creates an empty lookup with zero outcomes recorded.
    pub const fn new() -> Self {
        Self {
            wins: 0,
            ties: 0,
            losses: 0,
        }
    }

    /// Creates a lookup from explicit counters.
    pub const fn from_counts(wins: u64, ties: u64, losses: u64) -> Self {
        Self { wins, ties, losses }
    }

    /// Records a hero win.
    pub fn record_win(&mut self) {
        self.wins += 1;
    }

    /// Records a tie.
    pub fn record_tie(&mut self) {
        self.ties += 1;
    }

    /// Records a hero loss.
    pub fn record_loss(&mut self) {
        self.losses += 1;
    }

    /// Merges another lookup into this one.
    pub fn merge(&mut self, other: Self) {
        self.wins += other.wins;
        self.ties += other.ties;
        self.losses += other.losses;
    }

    /// Number of recorded wins.
    pub const fn wins(self) -> u64 {
        self.wins
    }

    /// Number of recorded ties.
    pub const fn ties(self) -> u64 {
        self.ties
    }

    /// Number of recorded losses.
    pub const fn losses(self) -> u64 {
        self.losses
    }

    /// Total number of outcomes.
    pub const fn samples(self) -> u64 {
        self.wins + self.ties + self.losses
    }

    /// Hero equity in the `0.0..=1.0` range.
    ///
    /// Returns `None` when no outcomes have been recorded.
    pub fn equity(self) -> Option<f64> {
        let samples = self.samples();
        if samples == 0 {
            return None;
        }

        let score = self.wins as f64 + (self.ties as f64 * 0.5);
        Some(score / samples as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::EquityLookup;

    #[test]
    fn computes_equity_from_counts() {
        let lookup = EquityLookup::from_counts(3, 2, 1);
        assert_eq!(lookup.samples(), 6);
        assert_eq!(lookup.equity(), Some(4.0 / 6.0));
    }

    #[test]
    fn merge_accumulates_outcomes() {
        let mut left = EquityLookup::from_counts(1, 1, 1);
        left.merge(EquityLookup::from_counts(2, 0, 1));

        assert_eq!(left.wins(), 3);
        assert_eq!(left.ties(), 1);
        assert_eq!(left.losses(), 2);
    }
}
