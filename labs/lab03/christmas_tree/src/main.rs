use serde::Deserialize;
use std::collections::VecDeque;
use std::io;
use std::ops::Deref;

#[derive(Debug, Deserialize)]
enum Instruction {
    Set(i32),
    Left,
    Right,
    Reset,
}

#[derive(Debug)]
struct Light {
    left: Option<Box<Light>>,
    right: Option<Box<Light>>,
    brightness: i32,
}

fn get_instructions_from_stdin() -> VecDeque<Instruction> {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).unwrap();
    ron::from_str(&instructions).unwrap()
}

fn traverse_tree(light: &Light) -> (i32, i32) {
    let mut cur_nodes = 1;
    let mut total_values = light.brightness;

    // traverse left nodes
    if let Some(ref left) = light.left {
        let (n, v) = traverse_tree(left.deref());
        cur_nodes += n;
        total_values += v;
    }

    // traverse right nodes
    if let Some(ref right) = light.right {
        let (n, v) = traverse_tree(right.deref());
        cur_nodes += n;
        total_values += v;
    }

    (cur_nodes, total_values)
}

fn main() {
    let instructions = get_instructions_from_stdin();
    let mut light = Light {
        left: None,
        right: None,
        brightness: 0,
    };
    // generate the tree one by one
    let mut cur_light = &mut light;
    for (_index, instruction) in instructions.iter().enumerate() {
        match instruction {
            Instruction::Set(val) => {
                cur_light.brightness = *val;
            }
            Instruction::Left => {
                if cur_light.left.is_none() {
                    cur_light.left = Some(Box::new(Light {
                        left: None,
                        right: None,
                        brightness: 0,
                    }));
                }
                cur_light = cur_light.left.as_mut().unwrap();
            }
            Instruction::Right => {
                if cur_light.right.is_none() {
                    cur_light.right = Some(Box::new(Light {
                        left: None,
                        right: None,
                        brightness: 0,
                    }));
                }
                cur_light = cur_light.right.as_mut().unwrap();
            }
            Instruction::Reset => cur_light = &mut light,
        }
    }
    let (total_nodes, total_lights) = traverse_tree(&light);
    println!("{}", total_lights / total_nodes);
}
