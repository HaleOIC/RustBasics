use core::f32;
use std::ops::Sub;

/// Returns the first element of a slice.
/// ```
/// use pointy::first;
/// let list = vec![1, 2, 3];
/// assert_eq!(&1, first(&list));
/// ```
///
/// ```
/// use pointy::first;
/// let list = vec!["a", "b", "c"];
/// assert_eq!(&"a", first(&list));
/// ```
// TODO: implement the below function
// You will need to change its signature
pub fn first<T>(list: &Vec<T>) -> &T {
    &list[0]
}

/// A generic point struct.
///
/// ```
/// use pointy::Point;
/// let point = Point { x: "string", y: "another" };
/// assert_eq!("string", point.x);
/// assert_eq!("another", point.y);
/// ```
///
/// ```
/// use pointy::Point;
/// let point = Point { x: 1.0, y: 2.0 };
/// assert_eq!(1.0, point.x);
/// assert_eq!(2.0, point.y);
/// ```
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Sub<Output = T> + Copy + Into<f32>,
{
    /// Returns the distance between two points.
    /// ```
    /// use pointy::Point;
    /// let point1 = Point { x: 1.0, y: 2.0 };
    /// let point2 = Point { x: 3.0, y: 4.0 };
    /// assert_eq!(2.828427, point1.distance(&point2));
    /// ```
    ///
    /// ```
    /// use pointy::Point;
    /// let point1 = Point::new(1.0, 2.0);
    /// assert_eq!(1.0, point1.x);
    /// ```
    pub fn distance(&self, second: &Point<T>) -> f32 {
        let x_diff: f32 = (self.x - second.x).into();
        let y_diff: f32 = (self.y - second.y).into();
        (x_diff * x_diff + y_diff * y_diff).sqrt()
    }

    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
