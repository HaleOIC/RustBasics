use std::ops::{Add, AddAssign, Mul};

/// Represents a 2D direction.
#[derive(Debug, PartialEq)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    /// Creates a new direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::direction::Direction;
    /// let dir = Direction::new(3, 4);
    /// assert_eq!(dir.x, 3);
    /// assert_eq!(dir.y, 4);
    /// ```
    pub fn new(x: i32, y: i32) -> Self {
        Direction { x, y }
    }
}

/// Represents cardinal directions.
#[derive(Debug, PartialEq)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl Add for Direction {
    type Output = Direction;

    /// Adds two directions.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::direction::Direction;
    /// let dir1 = Direction::new(1, 2);
    /// let dir2 = Direction::new(3, 4);
    /// let result = dir1 + dir2;
    /// assert_eq!(result.x, 4);
    /// assert_eq!(result.y, 6);
    /// ```
    fn add(self, other: Direction) -> Direction {
        Direction {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Direction {
    /// Adds and assigns a direction to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::direction::Direction;
    /// let mut dir1 = Direction::new(1, 2);
    /// let dir2 = Direction::new(3, 4);
    /// dir1 += dir2;
    /// assert_eq!(dir1.x, 4);
    /// assert_eq!(dir1.y, 6);
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl From<CardinalDirection> for Direction {
    /// Converts a cardinal direction to a direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::direction::{Direction, CardinalDirection};
    /// let dir: Direction = CardinalDirection::North.into();
    /// assert_eq!(dir.x, 0);
    /// assert_eq!(dir.y, 1);
    /// ```
    fn from(dir: CardinalDirection) -> Direction {
        match dir {
            CardinalDirection::North => Direction { x: 0, y: 1 },
            CardinalDirection::East => Direction { x: 1, y: 0 },
            CardinalDirection::South => Direction { x: 0, y: -1 },
            CardinalDirection::West => Direction { x: -1, y: 0 },
        }
    }
}

impl Mul<i32> for Direction {
    type Output = Direction;

    /// Multiplies a direction by a scalar.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::direction::Direction;
    /// let dir = Direction::new(1, 2);
    /// let result = dir * 3;
    /// assert_eq!(result.x, 3);
    /// assert_eq!(result.y, 6);
    /// ```
    fn mul(self, rhs: i32) -> Direction {
        Direction {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_new() {
        let dir = Direction::new(3, 4);
        assert_eq!(dir.x, 3);
        assert_eq!(dir.y, 4);
    }

    #[test]
    fn test_direction_add() {
        let dir1 = Direction::new(1, 2);
        let dir2 = Direction::new(3, 4);
        let result = dir1 + dir2;
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }

    #[test]
    fn test_direction_add_assign() {
        let mut dir1 = Direction::new(1, 2);
        let dir2 = Direction::new(3, 4);
        dir1 += dir2;
        assert_eq!(dir1.x, 4);
        assert_eq!(dir1.y, 6);
    }

    #[test]
    fn test_direction_from_cardinal_direction() {
        let dir: Direction = CardinalDirection::North.into();
        assert_eq!(dir.x, 0);
        assert_eq!(dir.y, 1);

        let dir: Direction = CardinalDirection::East.into();
        assert_eq!(dir.x, 1);
        assert_eq!(dir.y, 0);

        let dir: Direction = CardinalDirection::South.into();
        assert_eq!(dir.x, 0);
        assert_eq!(dir.y, -1);

        let dir: Direction = CardinalDirection::West.into();
        assert_eq!(dir.x, -1);
        assert_eq!(dir.y, 0);
    }
}
