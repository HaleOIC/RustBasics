use crate::direction::Direction;
use std::convert::From;
use std::default::Default;
use std::ops::{Add, AddAssign};

/// Represent a 2D coordinate.
#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    /// Create a new coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// let coord = Coordinate::new(3, 4);
    /// assert_eq!(coord.x, 3);
    /// assert_eq!(coord.y, 4);
    /// ```
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    /// Can find if a Coordinate is within a rectangle defined by two other Coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// let point = Coordinate { x: 3, y: 4 };
    /// let corner1 = Coordinate { x: 2, y: 3 };
    /// let corner2 = Coordinate { x: 5, y: 6 };
    /// assert!(point.within(&corner1, &corner2));
    ///
    /// let outside_point = Coordinate { x: 6, y: 7 };
    /// assert!(!outside_point.within(&corner1, &corner2));
    /// ```
    pub fn within(&self, y1: &Coordinate, y2: &Coordinate) -> bool {
        let (x_min, x_max) = if y1.x < y2.x {
            (y1.x, y2.x)
        } else {
            (y2.x, y1.x)
        };
        let (y_min, y_max) = if y1.y < y2.y {
            (y1.y, y2.y)
        } else {
            (y2.y, y1.y)
        };

        self.x >= x_min && self.x <= x_max && self.y >= y_min && self.y <= y_max
    }

    /// Returns the Manhattan distance between two coordinates.
    /// ```
    /// # use ws04::coordinate::Coordinate;
    /// let coord1 = Coordinate::new(1, 2);
    /// let coord2 = Coordinate::new(3, 4);
    /// assert_eq!(coord1.manhattan_distance(&coord2), 4);
    /// ```
    pub fn manhattan_distance(&self, other: &Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Default for Coordinate {
    /// Returns a default coordinate at (0, 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// let coord = Coordinate::default();
    /// assert_eq!(coord.x, 0);
    /// assert_eq!(coord.y, 0);
    /// ```
    fn default() -> Self {
        Coordinate { x: 0, y: 0 }
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    /// Adds two coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// let coord1 = Coordinate::new(1, 2);
    /// let coord2 = Coordinate::new(3, 4);
    /// let result = coord1 + coord2;
    /// assert_eq!(result.x, 4);
    /// assert_eq!(result.y, 6);
    /// ```
    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    /// Adds a direction to a coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// use ws04::direction::Direction;
    /// let coord = Coordinate::new(1, 2);
    /// let dir = Direction::new(3, 4);
    /// let result = coord + dir;
    /// assert_eq!(result.x, 4);
    /// assert_eq!(result.y, 6);
    /// ```
    fn add(self, rhs: Direction) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<Direction> for Coordinate {
    /// Converts a direction to a coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// use ws04::direction::Direction;
    /// let dir = Direction::new(3, 4);
    /// let coord: Coordinate = dir.into();
    /// assert_eq!(coord.x, 3);
    /// assert_eq!(coord.y, 4);
    /// ```
    fn from(dir: Direction) -> Coordinate {
        Coordinate { x: dir.x, y: dir.y }
    }
}

impl AddAssign for Coordinate {
    /// Adds and assigns a coordinate to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// let mut coord1 = Coordinate::new(1, 2);
    /// let coord2 = Coordinate::new(3, 4);
    /// coord1 += coord2;
    /// assert_eq!(coord1.x, 4);
    /// assert_eq!(coord1.y, 6);
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_within() {
        let point = Coordinate { x: 3, y: 4 };
        let corner1 = Coordinate { x: 2, y: 3 };
        let corner2 = Coordinate { x: 5, y: 6 };
        assert!(point.within(&corner1, &corner2));

        let outside_point = Coordinate { x: 6, y: 7 };
        assert!(!outside_point.within(&corner1, &corner2));
    }

    #[test]
    fn test_coordinate_add() {
        let coord1 = Coordinate::new(1, 2);
        let coord2 = Coordinate::new(3, 4);
        let result = coord1 + coord2;
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }

    #[test]
    fn test_coordinate_add_assign() {
        let mut coord1 = Coordinate::new(1, 2);
        let coord2 = Coordinate::new(3, 4);
        coord1 += coord2;
        assert_eq!(coord1.x, 4);
        assert_eq!(coord1.y, 6);
    }

    #[test]
    fn test_coordinate_default() {
        let coord = Coordinate::default();
        assert_eq!(coord.x, 0);
        assert_eq!(coord.y, 0);
    }

    #[test]
    fn test_coordinate_from_direction() {
        // Assuming Direction is defined elsewhere
        use crate::direction::Direction;
        let dir = Direction::new(3, 4);
        let coord: Coordinate = dir.into();
        assert_eq!(coord.x, 3);
        assert_eq!(coord.y, 4);
    }

    #[test]
    fn test_coordinate_add_direction() {
        // Assuming Direction is defined elsewhere
        use crate::direction::Direction;
        let coord = Coordinate::new(1, 2);
        let dir = Direction::new(3, 4);
        let result = coord + dir;
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }
}
