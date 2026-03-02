//! Two-dimensional grid range helpers.

/// A point in the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPoint {
    pub x: i64,
    pub y: i64,
}

/// A half-open 2D range: `[x_start, x_end) × [y_start, y_end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridRange {
    pub x_start: i64,
    pub x_end: i64,
    pub y_start: i64,
    pub y_end: i64,
}

impl GridRange {
    /// Returns a new grid range when dimensions are valid.
    pub fn new(x_start: i64, x_end: i64, y_start: i64, y_end: i64) -> Option<Self> {
        if x_start >= x_end || y_start >= y_end {
            return None;
        }

        Some(Self {
            x_start,
            x_end,
            y_start,
            y_end,
        })
    }

    /// Returns the width of this grid range.
    pub fn width(&self) -> u64 {
        (self.x_end - self.x_start) as u64
    }

    /// Returns the height of this grid range.
    pub fn height(&self) -> u64 {
        (self.y_end - self.y_start) as u64
    }

    /// Returns the number of points in this grid range.
    pub fn area(&self) -> u64 {
        self.width() * self.height()
    }

    /// Returns `true` if `point` is inside the half-open grid range.
    pub fn contains(&self, point: GridPoint) -> bool {
        (self.x_start..self.x_end).contains(&point.x)
            && (self.y_start..self.y_end).contains(&point.y)
    }

    /// Iterates points in row-major order.
    pub fn iter(&self) -> impl Iterator<Item = GridPoint> + '_ {
        (self.y_start..self.y_end)
            .flat_map(|y| (self.x_start..self.x_end).map(move |x| GridPoint { x, y }))
    }
}

#[cfg(test)]
mod tests {
    use super::{GridPoint, GridRange};

    #[test]
    fn computes_dimensions() {
        let grid = GridRange::new(0, 3, 0, 2).unwrap();
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.area(), 6);
    }

    #[test]
    fn iterates_row_major() {
        let grid = GridRange::new(1, 3, 2, 4).unwrap();
        let points: Vec<_> = grid.iter().collect();

        assert_eq!(
            points,
            vec![
                GridPoint { x: 1, y: 2 },
                GridPoint { x: 2, y: 2 },
                GridPoint { x: 1, y: 3 },
                GridPoint { x: 2, y: 3 },
            ]
        );
    }
}
