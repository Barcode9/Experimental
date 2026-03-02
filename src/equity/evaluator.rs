//! Lightweight equity evaluator over scalar hand strengths.

use crate::equity::lookup::EquityLookup;

/// Determines the outcome between hero and villain values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchResult {
    HeroWin,
    Tie,
    HeroLoss,
}

/// Evaluates equity by comparing pre-computed strength scores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EquityEvaluator {
    lookup: EquityLookup,
}

impl EquityEvaluator {
    /// Creates an empty evaluator.
    pub const fn new() -> Self {
        Self {
            lookup: EquityLookup::new(),
        }
    }

    /// Returns the aggregated lookup this evaluator has recorded.
    pub const fn lookup(self) -> EquityLookup {
        self.lookup
    }

    /// Compares hero and villain strengths and records the result.
    pub fn evaluate_pair(&mut self, hero_strength: u32, villain_strength: u32) -> MatchResult {
        let outcome = if hero_strength > villain_strength {
            MatchResult::HeroWin
        } else if hero_strength < villain_strength {
            MatchResult::HeroLoss
        } else {
            MatchResult::Tie
        };

        self.record(outcome);
        outcome
    }

    /// Records a known match result.
    pub fn record(&mut self, outcome: MatchResult) {
        match outcome {
            MatchResult::HeroWin => self.lookup.record_win(),
            MatchResult::Tie => self.lookup.record_tie(),
            MatchResult::HeroLoss => self.lookup.record_loss(),
        }
    }

    /// Current hero equity.
    pub fn equity(self) -> Option<f64> {
        self.lookup.equity()
    }
}

#[cfg(test)]
mod tests {
    use super::{EquityEvaluator, MatchResult};

    #[test]
    fn compares_strength_values() {
        let mut evaluator = EquityEvaluator::new();

        assert_eq!(evaluator.evaluate_pair(10, 7), MatchResult::HeroWin);
        assert_eq!(evaluator.evaluate_pair(7, 7), MatchResult::Tie);
        assert_eq!(evaluator.evaluate_pair(3, 8), MatchResult::HeroLoss);

        let lookup = evaluator.lookup();
        assert_eq!(lookup.wins(), 1);
        assert_eq!(lookup.ties(), 1);
        assert_eq!(lookup.losses(), 1);
    }

    #[test]
    fn computes_running_equity() {
        let mut evaluator = EquityEvaluator::new();
        evaluator.record(MatchResult::HeroWin);
        evaluator.record(MatchResult::HeroWin);
        evaluator.record(MatchResult::Tie);
        evaluator.record(MatchResult::HeroLoss);

        assert_eq!(evaluator.equity(), Some(0.625));
    }
}
