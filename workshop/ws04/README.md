
# README

**Tasks:**

- [X] Can add (including += ) Directions and Coordinates.
- [X] Can create Directions from CardinalDirections.
- [X] Can find if a Coordinate is within a rectange defined by two other Coordinates.
- [X] Has documentation for every public type.
- [X] Has at least one doctest for every public type.
- [X] Has tests to check internal functionality. We suggest one test per method/function.
- [X] Bonus: Does anything other features you think might be useful for the assignment.
- [X] Bonus: Finds the distance between two points.
- [X] Bonus: Implements scalar multiplication on Directions.

## tests

```shell
(py38) ➜  ws04 git:(main) ✗ cargo test
   Compiling ws04 v0.1.0 (/home/hale/self-learning/UNSW_COMP6991/workshop/ws04)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running unittests src/lib.rs (target/debug/deps/ws04-889ed0efd9d22aff)

running 12 tests
test coordinate::tests::test_coordinate_add_direction ... ok
test coordinate::tests::test_coordinate_add ... ok
test coordinate::tests::test_coordinate_add_assign ... ok
test coordinate::tests::test_coordinate_default ... ok
test coordinate::tests::test_coordinate_from_direction ... ok
test coordinate::tests::test_coordinate_within ... ok
test direction::tests::test_direction_add ... ok
test direction::tests::test_direction_add_assign ... ok
test direction::tests::test_direction_from_cardinal_direction ... ok
test direction::tests::test_direction_new ... ok
test tests::it_works ... ok
test tests::test_within ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ws04

running 13 tests
test src/coordinate.rs - coordinate::Coordinate::add (line 115) ... ok
test src/direction.rs - direction::Direction::add_assign (line 63) ... ok
test src/coordinate.rs - coordinate::Coordinate::from (line 137) ... ok
test src/coordinate.rs - coordinate::Coordinate::default (line 74) ... ok
test src/coordinate.rs - coordinate::Coordinate::manhattan_distance (line 58) ... ok
test src/coordinate.rs - coordinate::Coordinate::within (line 32) ... ok
test src/coordinate.rs - coordinate::Coordinate::new (line 18) ... ok
test src/direction.rs - direction::Direction::add (line 42) ... ok
test src/direction.rs - direction::Direction::from (line 84) ... ok
test src/coordinate.rs - coordinate::Coordinate::add_assign (line 155) ... ok
test src/coordinate.rs - coordinate::Coordinate::add (line 92) ... ok
test src/direction.rs - direction::Direction::new (line 15) ... ok
test src/direction.rs - direction::Direction::mul (line 107) ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.32s
```
