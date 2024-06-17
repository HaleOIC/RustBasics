use std::rc::Rc;

use simulator_lib::directions::{coordinate::Coordinate, direction::Direction};
use simulator_lib::{
    start_server, Asteroid, GravityObject, Planet, Pulsar, SelectiveGravityAsteroid,
};
fn main() {
    let mut _objects: Vec<Rc<dyn GravityObject>> = vec![
        Rc::new(Planet::new(Coordinate::new(550, 550), 40, 200)),
        Rc::new(Pulsar::new(Coordinate::new(700, 550), 40, 1, 300, 3)),
        Rc::new(Asteroid::new(
            Coordinate::new(300, 300),
            Direction::new(-30, 40),
        )),
        Rc::new(SelectiveGravityAsteroid::new(
            Coordinate::new(950, 950),
            Direction::new(30, -40),
        )),
        Rc::new(Asteroid::new(
            Coordinate::new(600, 250),
            Direction::new(-30, 40),
        )),
        Rc::new(SelectiveGravityAsteroid::new(
            Coordinate::new(300, 750),
            Direction::new(30, -40),
        )),
        Rc::new(Asteroid::new(
            Coordinate::new(200, 350),
            Direction::new(-30, 40),
        )),
        Rc::new(SelectiveGravityAsteroid::new(
            Coordinate::new(750, 950),
            Direction::new(30, -40),
        )),
    ];

    println!("Starting server. Open phys_simulation.html to see the simulation.");
    start_server("localhost:16991", _objects);
}

// ObjectType::Planet(Planet {
//     coordinate: Coordinate::new(500, 500),
//     weight: 50,
// }),
// ObjectType::Asteroid(Asteroid {
//     coordinate: Coordinate::new(250, 250),
//     velocity: Direction { x: 30, y: -40 },
// }),
// ObjectType::Asteroid(Asteroid {
//     coordinate: Coordinate::new(750, 750),
//     velocity: Direction { x: -30, y: 40 },
// }),
