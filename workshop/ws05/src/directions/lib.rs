pub mod coordinate;
pub mod direction;

#[cfg(test)]
mod tests {
    use crate::coordinate::Coordinate;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_within() {
        let point = Coordinate { x: 3, y: 4 };
        let corner1 = Coordinate { x: 2, y: 3 };
        let corner2 = Coordinate { x: 5, y: 6 };
        assert!(point.within(&corner1, &corner2));

        let outside_point = Coordinate { x: 6, y: 7 };
        assert!(!outside_point.within(&corner1, &corner2));
    }
}
