//! This module contains the definition of the statements that can be executed by the turtle.
//! Each statement should make sure it can be executed
//! and should be able to execute itself by given its runtime values.

use unsvg::{get_end_coordinates, Image};

use crate::{expression::Expression, pen::Pen};
use std::collections::HashMap;

/// The trait that all statements should implement.
/// Each statement should be able to execute itself by given its runtime values.
/// The runtime values are the values of the variables that are used in the statement.
/// The runtime values are stored in a hashmap.
/// The statement should also be able to update the runtime values, pen and image if needed.
pub trait Statement {
    fn execute(&self, values: &mut HashMap<String, f32>, pen: &mut Pen, image: &mut Image) -> bool;
}

/// Statement with no parameters.
/// The statement should be able to execute itself by given its runtime values.
/// It contains two kind of statements:
/// 1. PENUP
/// 2. PENDOWN
pub struct NoParaStatement {
    command: String,
}
impl NoParaStatement {
    pub fn new(tokens: &Vec<String>, start: usize) -> Option<(NoParaStatement, usize)> {
        let command = String::from(&tokens[start][..]);
        Some((NoParaStatement { command }, start))
    }
}

impl Statement for NoParaStatement {
    fn execute(
        &self,
        _values: &mut HashMap<String, f32>,
        pen: &mut Pen,
        _image: &mut Image,
    ) -> bool {
        match self.command.as_str() {
            "PENUP" => {
                pen.set_on_image(false);
                true
            }
            "PENDOWN" => {
                pen.set_on_image(true);
                true
            }
            _ => false,
        }
    }
}

/// Statement with one parameter of type Expression.
/// The statement should be able to execute itself by given its runtime values.
/// It contains four kind of statements:
/// 1. FORWARD [numpixels:f32]
/// 2. BACK [numpixels:f32]
/// 3. LEFT [numpixels:f32]
/// 4. RIGHT [numpixels:f32]
/// 5. TURN [degrees:f32]
/// 6. SETHEADING [degrees:f32]
/// 7. SETX [location:f32]
/// 8. SETY [location:f32]
/// 9. SETPENCOLOR [colorcode:f32]
pub struct UniStatement {
    command: String,
    arg: Expression,
}

impl UniStatement {
    pub fn new(tokens: &Vec<String>, start: usize) -> Option<(UniStatement, usize)> {
        let command = String::from(&tokens[start][..]);
        let (arg, end) = Expression::parse_expression(tokens, start + 1)?;
        Some((UniStatement { command, arg }, end))
    }
}

impl Statement for UniStatement {
    fn execute(&self, values: &mut HashMap<String, f32>, pen: &mut Pen, image: &mut Image) -> bool {
        let val = match self.arg.evaluate(values, &pen) {
            Some(val) => val,
            None => return false,
        };
        let cur_x = pen.get_x();
        let cur_y = pen.get_y();
        match self.command.as_str() {
            "FORWARD" => {
                let (new_x, new_y) = if pen.get_on_image() {
                    image
                        .draw_simple_line(cur_x, cur_y, pen.get_degree(), val, pen.get_color())
                        .map_err(|e| panic!("{}", e))
                        .unwrap()
                } else {
                    get_end_coordinates(cur_x, cur_y, pen.get_degree(), val)
                };
                pen.set_x(new_x);
                pen.set_y(new_y);
                true
            }
            "BACK" => {
                pen.turn_degree(180);
                let (new_x, new_y) = if pen.get_on_image() {
                    image
                        .draw_simple_line(cur_x, cur_y, pen.get_degree(), val, pen.get_color())
                        .map_err(|e| panic!("{}", e))
                        .unwrap()
                } else {
                    get_end_coordinates(cur_x, cur_y, pen.get_degree(), val)
                };
                pen.set_x(new_x);
                pen.set_y(new_y);
                pen.turn_degree(-180);
                true
            }
            "LEFT" => {
                pen.turn_degree(270);
                let (new_x, new_y) = if pen.get_on_image() {
                    image
                        .draw_simple_line(cur_x, cur_y, pen.get_degree(), val, pen.get_color())
                        .map_err(|e| panic!("{}", e))
                        .unwrap()
                } else {
                    get_end_coordinates(cur_x, cur_y, pen.get_degree(), val)
                };
                pen.set_x(new_x);
                pen.set_y(new_y);
                pen.turn_degree(-270);
                true
            }
            "RIGHT" => {
                pen.turn_degree(90);
                let (new_x, new_y) = if pen.get_on_image() {
                    image
                        .draw_simple_line(cur_x, cur_y, pen.get_degree(), val, pen.get_color())
                        .map_err(|e| panic!("{}", e))
                        .unwrap()
                } else {
                    get_end_coordinates(cur_x, cur_y, pen.get_degree(), val)
                };
                pen.set_x(new_x);
                pen.set_y(new_y);
                pen.turn_degree(-90);
                true
            }
            "TURN" => {
                pen.turn_degree(val as i32);
                true
            }
            "SETHEADING" => {
                pen.set_degree(val as i32);
                true
            }
            "SETX" => {
                pen.set_x(val);
                true
            }
            "SETY" => {
                pen.set_y(val);
                true
            }
            "SETPENCOLOR" => {
                return pen.set_color(val as usize);
            }
            _ => false,
        }
    }
}

/// Statement with two parameters of type Expression.
/// The statement should be able to execute itself by given its runtime values.
/// It contains one kind of statement:
/// 1. MAKE [arg1] [arg2]
/// 2. ADDASSIGN [variable] [expression]
pub struct BinaryStatement {
    command: String,
    variable: String,
    arg: Expression,
}

impl BinaryStatement {
    pub fn new(tokens: &Vec<String>, start: usize) -> Option<(BinaryStatement, usize)> {
        let command = String::from(&tokens[start][..]);

        // make sure not over range
        if start + 2 >= tokens.len() {
            return None;
        }

        // parse variable and expression
        if !tokens[start + 1].starts_with("\"") {
            return None;
        }
        let variable = String::from(&tokens[start + 1][1..]);
        let (arg, end) = Expression::parse_expression(tokens, start + 2)?;
        Some((BinaryStatement { command, variable, arg }, end))
    }
}

impl Statement for BinaryStatement {
    fn execute(&self, values: &mut HashMap<String, f32>, pen: &mut Pen, _image: &mut Image) -> bool {
        let variable = &self.variable;
        let val = match self.arg.evaluate(values, &pen) {
            Some(val) => val,
            None => return false,
        };
        match self.command.as_str() {
            "MAKE" => {
                values.insert(variable.to_string(), val);
                true
            }
            "ADDASSIGN" => {
                if let Some(v) = values.get_mut(variable) {
                    *v += val;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}




// pub struct IfStatement {
//     condition: Expression,
//     commands: Vec<Box<dyn Statement>>,
// }

// pub struct WhileStatement {
//     condition: Expression,
//     commands: Vec<Box<dyn Statement>>,
// }

// pub struct ProcedureStatement {
//     name: String,
//     commands: Vec<Box<dyn Statement>>,
// }

// impl Statement for UniStatement {
//     fn execute(&self, values: &HashMap<String, f32>) -> Result<(), ()> {
//         // match self.command.as_str() {
//         //     "FD" => {
//         //         let val = self.arg.evaluate(values);
//         //         println!("FD {}", val);
//         //         Ok(())
//         //     }
//         //     "BK" => {
//         //         let val = self.arg.evaluate(values);
//         //         println!("BK {}", val);
//         //         Ok(())
//         //     }
//         //     "RT" => {
//         //         let val = self.arg.evaluate(values);
//         //         println!("RT {}", val);
//         //         Ok(())
//         //     }
//         //     "LT" => {
//         //         let val = self.arg.evaluate(values);
//         //         println!("LT {}", val);
//         //         Ok(())
//         //     }
//         //     _ => Err(()),
//         // }
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expression, pen::Pen};
    use unsvg::Image;

    #[test]
    fn test_no_para_statement() {
        // Setup initial pen state and image
        let mut pen = Pen::new(100, 70); // Assuming there is a default state for Pen
        let mut image = Image::new(100, 70); // Assuming a method to create a new image

        // Create a PENUP statement and execute it
        let penup_statement = NoParaStatement {
            command: "PENUP".to_string(),
        };
        assert_eq!(
            penup_statement.execute(&mut HashMap::new(), &mut pen, &mut image),
            true
        );
        assert_eq!(pen.get_on_image(), false); // Assuming there is a method to check if pen is on image

        // Create a PENDOWN statement and execute it
        let pendown_statement = NoParaStatement {
            command: "PENDOWN".to_string(),
        };
        assert_eq!(
            pendown_statement.execute(&mut HashMap::new(), &mut pen, &mut image),
            true
        );
        assert_eq!(pen.get_on_image(), true); // Assuming there is a method to check if pen is on image
    }


    #[test]
    fn test_uni_statement_forward() {
        let mut values = HashMap::new();
        let mut pen = Pen::new(200, 200);
        let mut image = Image::new(200, 200);

        let command = "FORWARD".to_string();
        let expression = expression::Expression::Value(10.0);
        let uni_statement = UniStatement { command, arg: expression };

        assert!(uni_statement.execute(&mut values, &mut pen, &mut image));
        assert_eq!(pen.get_y(), 90.0); 
        assert_eq!(pen.get_x(), 100.0);
    }

    #[test]
    fn test_uni_statement_turn() {
        let mut values = HashMap::new();
        let mut pen = Pen::new(200, 200);
        let mut image = Image::new(200, 200);

        let command = "TURN".to_string();
        let expression = expression::Expression::Value(90.0);
        let uni_statement = UniStatement { command, arg: expression };

        assert!(uni_statement.execute(&mut values, &mut pen, &mut image));
        assert_eq!(pen.get_degree(), 90); // Assuming the initial heading is 0 degrees
    }

    #[test]
    fn test_uni_statement_setx() {
        let mut values = HashMap::new();
        let mut pen = Pen::new(200, 200);
        let mut image = Image::new(200, 200);

        let command = "SETX".to_string();
        let expression = expression::Expression::Value(100.0);
        let uni_statement = UniStatement { command, arg: expression };

        assert!(uni_statement.execute(&mut values, &mut pen, &mut image));
        assert_eq!(pen.get_x(), 100.0);
    }
}
