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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procedure_with_no_commands() {
        let tokens = vec!["TO", "ProcNoCmd", "\"var1", "END"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut index = 0;
        let definitions = HashMap::new();

        let result = ProcedureDefinition::new(&tokens, &mut index, &definitions);
        assert!(result.is_some());
        let proc_def = result.unwrap();
        assert_eq!(proc_def.name, "ProcNoCmd");
        assert_eq!(proc_def.variables, vec!["var1"]);
        assert!(proc_def.commands.is_empty());
    }

    #[test]
    fn test_invalid_token_sequence() {
        let tokens = vec!["TO", "ProcInvalid", "\"var2", "Oops", "END"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut index = 0;
        let definitions = HashMap::new();

        let result = ProcedureDefinition::new(&tokens, &mut index, &definitions);
        assert!(result.is_none());
    }

    #[test]
    fn test_missing_end_token() {
        let tokens = vec!["TO", "ProcMissingEnd", "\"var3", "DO_SOMETHING"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut index = 0;
        let definitions = HashMap::new();

        let result = ProcedureDefinition::new(&tokens, &mut index, &definitions);
        assert!(result.is_none());
    }
}
