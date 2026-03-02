//! Combining multiple weighted ranges.

use crate::ranges::WeightedRange;

/// A collection of weighted ranges with helper selection logic.
#[derive(Debug, Clone, Default)]
pub struct RangeCombo {
    ranges: Vec<WeightedRange>,
}

impl RangeCombo {
    /// Creates an empty combo.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a combo from pre-validated ranges.
    pub fn from_ranges(ranges: Vec<WeightedRange>) -> Self {
        Self { ranges }
    }

    /// Adds a new weighted range to this combo.
    ///
    /// Returns `None` if inputs do not form a valid [`WeightedRange`].
    pub fn push(&mut self, start: i64, end: i64, weight: u64) -> Option<()> {
        let range = WeightedRange::new(start, end, weight)?;
        self.ranges.push(range);
        Some(())
    }

    /// Returns all ranges in insertion order.
    pub fn ranges(&self) -> &[WeightedRange] {
        &self.ranges
    }

    /// Returns the sum of all weights.
    pub fn total_weight(&self) -> u64 {
        self.ranges.iter().map(|r| r.weight).sum()
    }

    /// Returns the first range selected by `ticket` in weighted order.
    ///
    /// `ticket` should normally be in `0..total_weight`. Larger values wrap by modulo.
    pub fn choose_by_weight(&self, ticket: u64) -> Option<WeightedRange> {
        let total_weight = self.total_weight();
        if total_weight == 0 {
            return None;
        }

        let mut cursor = ticket % total_weight;
        for range in &self.ranges {
            if cursor < range.weight {
                return Some(*range);
            }
            cursor -= range.weight;
        }

        None
    }

    /// Returns `true` if at least one range contains `value`.
    pub fn contains(&self, value: i64) -> bool {
        self.ranges.iter().any(|r| r.contains(value))
    }
}

#[cfg(test)]
mod tests {
    use super::RangeCombo;

    #[test]
    fn weighted_choice_prefers_ranges_by_ticket() {
        let mut combo = RangeCombo::new();
        combo.push(0, 5, 1).unwrap();
        combo.push(10, 20, 3).unwrap();

        let first = combo.choose_by_weight(0).unwrap();
        let second = combo.choose_by_weight(1).unwrap();
        let wrapped = combo.choose_by_weight(5).unwrap();

        assert_eq!(first.start, 0);
        assert_eq!(second.start, 10);
        assert_eq!(wrapped.start, 10);
    }

    #[test]
    fn contains_checks_all_ranges() {
        let mut combo = RangeCombo::new();
        combo.push(0, 2, 1).unwrap();
        combo.push(10, 12, 1).unwrap();

        assert!(combo.contains(0));
        assert!(combo.contains(11));
        assert!(!combo.contains(9));
    }
}
