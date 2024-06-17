pub mod directions;

use crate::directions::{coordinate::Coordinate, direction::Direction};

use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    rc::Rc,
};

use serde::{Deserialize, Serialize};

/// Objects return back to front-end to render
#[derive(Deserialize, Serialize)]
pub struct Circle {
    cx: i32,
    cy: i32,
    r: i32,
    stroke: String,
    fill: String,
    #[serde(rename = "stroke-width")]
    stroke_width: i32,
}

/// Utility function to calculate E-distance
fn get_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64).sqrt() as i32
}

fn calculate_distance(
    receiver: &dyn GravityReceiver,
    provider: &dyn GravitySource,
) -> (i32, Direction) {
    let planet_weight = provider.get_weight();
    let gravitational_constant = provider.get_gravity_constant();
    let coordinate = receiver.get_location();
    let other_location = provider.get_location();
    let distance = get_distance(
        coordinate.x,
        coordinate.y,
        other_location.x,
        other_location.y,
    );
    let r = distance * distance;
    (
        distance,
        Direction {
            x: -(coordinate.x - other_location.x) * planet_weight * gravitational_constant / r,
            y: -(coordinate.y - other_location.y) * planet_weight * gravitational_constant / r,
        },
    )
}

/// Task 7: GravityObjects
/// Combine both Gravitational Objects with One pub trait
pub trait GravityObject: CelestialObject {
    fn check_source(&self) -> Option<&dyn GravitySource>;
    fn check_receiver(&self) -> Option<&dyn GravityReceiver>;
    fn get_source(&mut self) -> Option<&mut dyn GravitySource>;
    fn get_receiver(&mut self) -> Option<&mut dyn GravityReceiver>;
}

impl GravityObject for Planet {
    fn get_source(&mut self) -> Option<&mut dyn GravitySource> {
        Some(self)
    }

    fn check_source(&self) -> Option<&dyn GravitySource> {
        Some(self)
    }

    fn check_receiver(&self) -> Option<&dyn GravityReceiver> {
        None
    }

    fn get_receiver(&mut self) -> Option<&mut dyn GravityReceiver> {
        None
    }
}

impl GravityObject for Pulsar {
    fn get_source(&mut self) -> Option<&mut dyn GravitySource> {
        Some(self)
    }

    fn check_source(&self) -> Option<&dyn GravitySource> {
        Some(self)
    }

    fn check_receiver(&self) -> Option<&dyn GravityReceiver> {
        None
    }

    fn get_receiver(&mut self) -> Option<&mut dyn GravityReceiver> {
        None
    }
}

impl GravityObject for Asteroid {
    fn get_source(&mut self) -> Option<&mut dyn GravitySource> {
        None
    }

    fn check_source(&self) -> Option<&dyn GravitySource> {
        None
    }

    fn check_receiver(&self) -> Option<&dyn GravityReceiver> {
        Some(self)
    }

    fn get_receiver(&mut self) -> Option<&mut dyn GravityReceiver> {
        Some(self)
    }
}

impl GravityObject for SelectiveGravityAsteroid {
    fn get_source(&mut self) -> Option<&mut dyn GravitySource> {
        None
    }

    fn check_source(&self) -> Option<&dyn GravitySource> {
        None
    }

    fn check_receiver(&self) -> Option<&dyn GravityReceiver> {
        Some(self)
    }

    fn get_receiver(&mut self) -> Option<&mut dyn GravityReceiver> {
        Some(self)
    }
}

///////////////////////// struct definitions //////////////
#[derive(Clone)]
pub struct Planet {
    coordinate: Coordinate,
    weight: i32,
    gravity_constant: i32,
}

impl Planet {
    pub fn new(c: Coordinate, w: i32, g: i32) -> Self {
        Planet {
            coordinate: c,
            weight: w,
            gravity_constant: g,
        }
    }
}

#[derive(Clone)]
pub struct Pulsar {
    coordinate: Coordinate,
    weight: i32,
    cur_state: i32,
    highest_gravity: i32,
    total_states: i32,
    direction: bool,
}

impl Pulsar {
    pub fn new(c: Coordinate, w: i32, cur_s: i32, t: i32, h: i32) -> Self {
        Pulsar {
            coordinate: c,
            weight: w,
            cur_state: cur_s,
            highest_gravity: h,
            total_states: t,
            direction: true,
        }
    }
}

#[derive(Clone)]
pub struct Asteroid {
    coordinate: Coordinate,
    velocity: Direction,
}

impl Asteroid {
    pub fn new(c: Coordinate, v: Direction) -> Self {
        Asteroid {
            coordinate: c,
            velocity: v,
        }
    }
}

#[derive(Clone)]
pub struct SelectiveGravityAsteroid {
    coordinate: Coordinate,
    velocity: Direction,
}

impl SelectiveGravityAsteroid {
    pub fn new(c: Coordinate, v: Direction) -> Self {
        SelectiveGravityAsteroid {
            coordinate: c,
            velocity: v,
        }
    }
}

/// Task4: refactor so that Planets and Asteriods
/// share a pub trait which defines their position
/// and allows conversion into a circle.
pub trait CelestialObject {
    fn as_circle(&self) -> Circle;
    fn get_location(&self) -> Coordinate;
}

impl CelestialObject for Planet {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }
    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: self.weight,
            stroke: "green".to_string(),
            fill: "black".to_string(),
            stroke_width: 3,
        }
    }
}

impl CelestialObject for Asteroid {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }

    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: 5,
            stroke: "green".to_string(),
            fill: "black".to_string(),
            stroke_width: 3,
        }
    }
}

impl CelestialObject for Pulsar {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }

    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: self.weight,
            stroke: "purple".to_string(),
            fill: "lightblue".to_string(),
            stroke_width: 3,
        }
    }
}

impl CelestialObject for SelectiveGravityAsteroid {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }

    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: 10,
            stroke: "yellow".to_string(),
            fill: "pink".to_string(),
            stroke_width: 3,
        }
    }
}

/// pub trait: Gravity Source
/// inner function for get gravity_constant
pub trait GravitySource: CelestialObject {
    fn get_gravity_constant(&self) -> i32;
    fn get_weight(&self) -> i32;
    fn change_gravity_constant(&mut self);
}

impl GravitySource for Planet {
    fn get_gravity_constant(&self) -> i32 {
        self.gravity_constant
    }
    fn get_weight(&self) -> i32 {
        self.weight
    }
    fn change_gravity_constant(&mut self) {}
}

impl GravitySource for Pulsar {
    fn get_gravity_constant(&self) -> i32 {
        (self.cur_state as f32 / self.total_states as f32 * self.highest_gravity as f32) as i32
    }
    fn get_weight(&self) -> i32 {
        self.weight
    }
    fn change_gravity_constant(&mut self) {
        if self.direction {
            self.cur_state += 1;
        } else {
            self.cur_state -= 1;
        }
        if self.cur_state == self.highest_gravity {
            self.direction = false;
        }
        if self.cur_state == 1 {
            self.direction = true;
        }
    }
}

/// Task 6: pub trait: Gravity Receiver
/// inner function for calculate affect to each receiver
pub trait GravityReceiver: CelestialObject {
    fn apply_gravity(&self, provider: &dyn GravitySource) -> Direction;
    fn make_move(&mut self, force: Direction);
}

impl GravityReceiver for Asteroid {
    fn apply_gravity(&self, provider: &dyn GravitySource) -> Direction {
        let (dist, force) = calculate_distance(self, provider);
        if dist > 500 {
            Direction { x: 0, y: 0 }
        } else {
            force
        }
    }

    fn make_move(&mut self, force: Direction) {
        self.velocity += force;
        self.coordinate.x += self.velocity.x;
        self.coordinate.y += self.velocity.y;
    }
}

impl GravityReceiver for SelectiveGravityAsteroid {
    fn apply_gravity(&self, provider: &dyn GravitySource) -> Direction {
        let (_, force) = calculate_distance(self, provider);
        force
    }
    fn make_move(&mut self, force: Direction) {
        self.velocity += force;
        self.coordinate.x += self.velocity.x;
        self.coordinate.y += self.velocity.y;
    }
}

fn apply_physics(mut objects: Vec<Rc<dyn GravityObject>>) -> Vec<Rc<dyn GravityObject>> {
    // split sources and receivers
    let gravity_sources: Vec<&dyn GravitySource> =
        objects.iter().filter_map(|o| (*o).check_source()).collect();
    let gravity_receivers: Vec<&dyn GravityReceiver> = objects
        .iter()
        .filter_map(|o| (*o).check_receiver())
        .collect();

    let mut forces = Vec::new();
    for recv in &gravity_receivers {
        let mut force = Direction::new(0, 0);
        for source in &gravity_sources {
            force += (*recv).apply_gravity(*source);
        }
        forces.push(force);
    }
    let mut index = 0;
    objects
        .iter_mut()
        .filter_map(|o| Rc::get_mut(o).unwrap().get_receiver())
        .for_each(|recv| {
            let force = forces.get(index).unwrap();
            recv.make_move(force.clone());
            index += 1;
        });
    objects
        .iter_mut()
        .filter_map(|o| Rc::get_mut(o).unwrap().get_source())
        .for_each(|src| src.change_gravity_constant());
    objects
}

fn handle_connection(
    mut stream: TcpStream,
    mut objects: Vec<Rc<dyn GravityObject>>,
) -> Vec<Rc<dyn GravityObject>> {
    objects = apply_physics(objects);

    let circles: Vec<Circle> = objects.iter().map(|o| o.as_circle()).collect();
    let contents = serde_json::to_string(&circles).unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let response = format!(
        "{status_line}\r\nContentType: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n{contents}\r\n"
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap();

    objects
}

pub fn start_server(uri: &str, mut objects: Vec<Rc<dyn GravityObject>>) -> ! {
    let listener = TcpListener::bind(uri).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        objects = handle_connection(stream, objects);
    }

    unreachable!()
}
