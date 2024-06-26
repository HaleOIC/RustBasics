use std::{collections::HashMap, rc::Rc};

use crate::statements::{parse_statement, Statement};

pub struct ProcedureDefinition {
    pub variables: Vec<String>,
    pub commands: Vec<Box<dyn Statement>>,
    pub name: String,
}


/// Parse the tokens into a procedure definition
impl ProcedureDefinition {
    pub fn new(
        tokens: &Vec<String>,
        start: &mut usize,
        definitions: &HashMap<String, Rc<ProcedureDefinition>>,
    ) -> Option<ProcedureDefinition> {
        // make sure it is procedure definition
        if &tokens[*start][..] != "TO" {
            return None;
        }

        // parse the function name and variables
        let mut variables = Vec::new();
        let mut i = *start + 1;
        let name = tokens[i].to_string();
        i += 1;
        while i < tokens.len() && tokens[i].starts_with("\"") {
            variables.push(tokens[i][1..].to_string());
            i += 1;
        }
        if i >= tokens.len() {
            return None;
        }

        // parse commands one by one
        let mut commands = Vec::new();
        while i < tokens.len() && tokens[i] != "END" {
            if let Some(statement) = parse_statement(tokens, &mut i, &definitions) {
                commands.push(statement);
            } else {
                return None;
            }
        }
        if i >= tokens.len() {
            return None;
        }

        // set index and return back
        *start = i + 1;
        Some(ProcedureDefinition {
            variables,
            name,
            commands,
        })
    }
}
