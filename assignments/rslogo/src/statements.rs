//! This module contains the definition of the statements that can be executed by the turtle.
//! Each statement should make sure it can be executed
//! and should be able to execute itself by given its runtime values.

use unsvg::{get_end_coordinates, Image};

use crate::{
    definition::ProcedureDefinition,
    expression::{Expression, Outcome},
    pen::Pen,
};
use std::{collections::HashMap, rc::Rc};

/// The trait that all statements should implement.
/// Each statement should be able to execute itself by given its runtime values.
/// The runtime values are the values of the variables that are used in the statement.
/// The runtime values are stored in a hashmap.
/// The statement should also be able to update the runtime values, pen and image if needed.
pub trait Statement {
    fn execute(
        &self,
        values: &mut HashMap<String, Outcome>,
        pen: &mut Pen,
        image: &mut Image,
    ) -> bool;
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
        _values: &mut HashMap<String, Outcome>,
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
    fn execute(
        &self,
        values: &mut HashMap<String, Outcome>,
        pen: &mut Pen,
        image: &mut Image,
    ) -> bool {
        let val = match self.arg.evaluate(values, &pen) {
            Some(val) => match val {
                Outcome::Value(val) => val,
                _ => return false,
            },
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
        Some((
            BinaryStatement {
                command,
                variable,
                arg,
            },
            end,
        ))
    }
}

impl Statement for BinaryStatement {
    fn execute(
        &self,
        values: &mut HashMap<String, Outcome>,
        pen: &mut Pen,
        _image: &mut Image,
    ) -> bool {
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
                    let value = match val {
                        Outcome::Value(val) => val,
                        _ => return false,
                    };
                    let new_val = match v {
                        Outcome::Value(ori) => Outcome::Value(*ori + value),
                        _ => return false,
                    };
                    *v = new_val;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

/// IF statement
pub struct IfStatement {
    condition: Expression,
    commands: Vec<Box<dyn Statement>>,
}

impl IfStatement {
    pub fn new(
        tokens: &Vec<String>,
        definitions: &HashMap<String, Rc<ProcedureDefinition>>,
        start: usize,
    ) -> Option<(IfStatement, usize)> {
        let (condition, end) = Expression::parse_expression(tokens, start + 1)?;
        let mut commands = Vec::new();
        let mut i = end + 1;

        // make sure next token is [
        if !tokens[i].starts_with("[") {
            return None;
        }
        i += 1; // shift to next token start
        while i < tokens.len() && !tokens[i].starts_with("]") {
            let statement = parse_statement(tokens, &mut i, &definitions)?;
            commands.push(statement);
        }
        if i >= tokens.len() {
            return None;
        }
        Some((
            IfStatement {
                condition,
                commands,
            },
            i,
        ))
    }
}

impl Statement for IfStatement {
    fn execute(
        &self,
        values: &mut HashMap<String, Outcome>,
        pen: &mut Pen,
        image: &mut Image,
    ) -> bool {
        let val = match self.condition.evaluate(values, &pen) {
            Some(val) => match val {
                Outcome::Bool(val) => val,
                _ => return false,
            },
            None => return false,
        };
        if val {
            for command in &self.commands {
                if !command.execute(values, pen, image) {
                    return false;
                }
            }
        }
        true
    }
}

// loop statement
pub struct WhileStatement {
    condition: Expression,
    commands: Vec<Box<dyn Statement>>,
}

impl WhileStatement {
    pub fn new(
        tokens: &Vec<String>,
        definitions: &HashMap<String, Rc<ProcedureDefinition>>,
        start: usize,
    ) -> Option<(WhileStatement, usize)> {
        let (condition, end) = Expression::parse_expression(tokens, start + 1)?;
        let mut commands = Vec::new();
        let mut i = end + 1;

        // make sure next token is [
        if !tokens[i].starts_with("[") {
            return None;
        }
        i += 1; // shift to next token start
        while i < tokens.len() && !tokens[i].starts_with("]") {
            let statement = parse_statement(tokens, &mut i, &definitions)?;
            commands.push(statement);
        }
        if i >= tokens.len() {
            return None;
        }
        Some((
            WhileStatement {
                condition,
                commands,
            },
            i,
        ))
    }
}

impl Statement for WhileStatement {
    fn execute(
        &self,
        values: &mut HashMap<String, Outcome>,
        pen: &mut Pen,
        image: &mut Image,
    ) -> bool {
        loop {
            let val = match self.condition.evaluate(values, &pen) {
                Some(val) => match val {
                    Outcome::Bool(val) => val,
                    _ => return false,
                },
                None => return false,
            };
            if val {
                for command in &self.commands {
                    if !command.execute(values, pen, image) {
                        return false;
                    }
                }
            } else {
                break;
            }
        }
        true
    }
}

/// procedure statement (function)
/// function can have any number of variables

pub struct ProcedureStatement {
    args: Vec<Expression>,
    body: Rc<ProcedureDefinition>,
}
impl ProcedureStatement {
    pub fn new(
        tokens: &Vec<String>,
        start: usize,
        body: &Rc<ProcedureDefinition>,
    ) -> Option<(ProcedureStatement, usize)> {
        // make sure it is procedure statement
        let mut i = start;
        let arg_num = body.variables.len();

        // // parse expression
        let mut args = Vec::new();
        for _ in 0..arg_num {
            let (arg, end) = Expression::parse_expression(tokens, i + 1)?;
            args.push(arg);
            i = end;
        }
        Some((
            ProcedureStatement {
                args,
                body: Rc::clone(&body),
            },
            i,
        ))
    }
}

impl Statement for ProcedureStatement {
    fn execute(
        &self,
        values: &mut HashMap<String, Outcome>,
        pen: &mut Pen,
        image: &mut Image,
    ) -> bool {
        let mut new_values = values.clone();
        for (i, arg) in self.args.iter().enumerate() {
            let val = match arg.evaluate(values, &pen) {
                Some(val) => val,
                None => return false,
            };
            new_values.insert(self.body.variables[i].clone(), val);
        }
        for command in &self.body.commands {
            if !command.execute(&mut new_values, pen, image) {
                return false;
            }
        }
        true
    }
}

pub fn parse_statement(
    tokens: &Vec<String>,
    index: &mut usize,
    definitions: &HashMap<String, Rc<ProcedureDefinition>>,
) -> Option<Box<dyn Statement>> {
    let cur_token = &tokens[*index];
    match cur_token.as_str() {
        "PENUP" | "PENDOWN" => {
            let (new_statement, new_index) = NoParaStatement::new(tokens, *index)?;
            *index = new_index + 1;
            Some(Box::new(new_statement))
        }
        "FORWARD" | "BACK" | "LEFT" | "RIGHT" | "SETPENCOLOR" | "TURN" | "SETHEADING" | "SETX"
        | "SETY" => {
            let (new_statement, new_index) = UniStatement::new(tokens, *index)?;
            *index = new_index + 1;
            Some(Box::new(new_statement))
        }
        "MAKE" | "ADDASSIGN" => {
            let (new_statement, new_index) = BinaryStatement::new(tokens, *index)?;
            *index = new_index + 1;
            Some(Box::new(new_statement))
        }
        "IF" => {
            let (new_statement, new_index) = IfStatement::new(tokens, definitions, *index)?;
            *index = new_index + 1;
            Some(Box::new(new_statement))
        }
        "WHILE" => {
            let (new_statement, new_index) = WhileStatement::new(tokens, definitions, *index)?;
            *index = new_index + 1;
            Some(Box::new(new_statement))
        }
        _ => {
            // maybe calling function
            if definitions.contains_key(cur_token) {
                let (new_statement, new_index) =
                    ProcedureStatement::new(tokens, *index, definitions.get(cur_token).unwrap())?;
                *index = new_index + 1;
                Some(Box::new(new_statement))
            } else {
                eprintln!("Unknown command: {}", cur_token);
                None
            }
        }
    }
}

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
        let uni_statement = UniStatement {
            command,
            arg: expression,
        };

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
        let uni_statement = UniStatement {
            command,
            arg: expression,
        };

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
        let uni_statement = UniStatement {
            command,
            arg: expression,
        };

        assert!(uni_statement.execute(&mut values, &mut pen, &mut image));
        assert_eq!(pen.get_x(), 100.0);
    }

    // Setup a mock function to replace real image and pen behavior in tests
    fn setup_pen_and_image() -> (Pen, Image) {
        (Pen::new(100, 100), Image::new(200, 200))
    }

    #[test]
    fn test_binary_statement_make() {
        let (mut pen, mut image) = setup_pen_and_image();
        let mut values = HashMap::new();
        let expression = Expression::Value(10.0);
        let make_statement = BinaryStatement {
            command: "MAKE".to_string(),
            variable: "var".to_string(),
            arg: expression,
        };

        assert!(make_statement.execute(&mut values, &mut pen, &mut image));
        assert_eq!(
            values.get("var"),
            Some(&Outcome::Value(10.0)),
            "Variable 'var' should be set to 10.0"
        );
    }

    #[test]
    fn test_if_statement_execution() {
        let (mut pen, mut image) = setup_pen_and_image();
        let mut values = HashMap::new();
        values.insert("cond".to_string(), Outcome::Bool(true));
        let condition = Expression::Variable("cond".to_string());
        let command = Box::new(NoParaStatement {
            command: "PENDOWN".to_string(),
        });
        let if_statement = IfStatement {
            condition,
            commands: vec![command],
        };

        assert!(if_statement.execute(&mut values, &mut pen, &mut image));
        assert!(
            pen.get_on_image(),
            "Pen should be down if condition is true"
        );
    }

    #[test]
    fn test_while_statement_execution() {
        let (mut pen, mut image) = setup_pen_and_image();
        let mut values = HashMap::new();
        values.insert("loop".to_string(), Outcome::Bool(true));
        let condition = Expression::Variable("loop".to_string());
        let command = Box::new(NoParaStatement {
            command: "PENDOWN".to_string(),
        });
        let while_statement = WhileStatement {
            condition,
            commands: vec![command],
        };

        // Set loop condition to false to avoid infinite loop in testing
        values.insert("loop".to_string(), Outcome::Bool(false));
        assert!(while_statement.execute(&mut values, &mut pen, &mut image));
        // Check state to ensure loop exited correctly
        assert!(
            !pen.get_on_image(),
            "Pen should remain up if condition is false initially"
        );
    }
}
