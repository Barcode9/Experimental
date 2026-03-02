//! Weighted one-dimensional range helpers.

/// A half-open integer range (`start..end`) with an associated weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeightedRange {
    pub start: i64,
    pub end: i64,
    pub weight: u64,
}

impl WeightedRange {
    /// Creates a new [`WeightedRange`].
    ///
    /// Returns `None` when `start >= end` or `weight == 0`.
    pub fn new(start: i64, end: i64, weight: u64) -> Option<Self> {
        if start >= end || weight == 0 {
            return None;
        }

        Some(Self { start, end, weight })
    }

    /// Returns the number of integer values in this half-open range.
    pub fn len(&self) -> u64 {
        (self.end - self.start) as u64
    }

    /// Returns `true` when the range is empty.
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Returns `true` if `value` is contained in this half-open range.
    pub fn contains(&self, value: i64) -> bool {
        (self.start..self.end).contains(&value)
    }

    /// Returns a copy with a new weight.
    ///
    /// Returns `None` if `weight == 0`.
    pub fn with_weight(self, weight: u64) -> Option<Self> {
        if weight == 0 {
            return None;
        }

        Some(Self { weight, ..self })
    }
}

#[cfg(test)]
mod tests {
    use super::WeightedRange;

    #[test]
    fn validates_inputs() {
        assert!(WeightedRange::new(0, 0, 1).is_none());
        assert!(WeightedRange::new(5, 1, 1).is_none());
        assert!(WeightedRange::new(0, 5, 0).is_none());
        assert!(WeightedRange::new(0, 5, 1).is_some());
    }

    #[test]
    fn contains_and_length_work() {
        let range = WeightedRange::new(10, 13, 7).unwrap();

        assert_eq!(range.len(), 3);
        assert!(range.contains(10));
        assert!(range.contains(12));
        assert!(!range.contains(13));
    }
}
