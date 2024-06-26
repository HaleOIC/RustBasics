use crate::statements::{parse_statement, Statement};
use crate::definition::ProcedureDefinition;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

pub struct Parser {
    lines: Vec<String>,
    tokens: Vec<String>,
    tokens_statements: Vec<String>,
    statements: Vec<Box<dyn Statement>>,
    definitions: HashMap<String, Rc<ProcedureDefinition>>,
}

impl Parser {
    /// Parse the given file into a parser
    /// # Arguments
    /// * `file_path` - A string slice that holds the path to the file
    /// # Returns
    /// * A Result containing the parser or an error message
    pub fn parse_file(file_path: &str) -> Result<Parser, String> {
        let mut lines: Vec<String> = Vec::new();
        let file = File::open(file_path).expect("Could not open given file!");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if let Some(pos) = line.find("//") {
                        lines.push(String::from(&line[..pos]));
                    } else {
                        lines.push(String::from(line));
                    }
                }
                Err(e) => println!("Error reading line: {}", e),
            }
        }
        Ok(Parser {
            lines,
            tokens: Vec::new(),
            tokens_statements: Vec::new(),
            statements: Vec::new(),
            definitions: HashMap::new(),
        })
    }

    /// parse lines into tokens stream
    pub fn into_tokens(&mut self) {
        self.tokens = self
            .lines
            .iter()
            .flat_map(|line| line.split_whitespace().map(String::from))
            .collect();
    }

    /// show the tokens
    pub fn show_statements(&self) -> &Vec<Box<dyn Statement>> {
        return &self.statements;
    }

    /// convert tokens into statements
    pub fn into_statements(&mut self) -> Option<bool> {        
        let mut index = 0;
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        while index < self.tokens_statements.len() {
            // parse statement
            if let Some(new_statement) = parse_statement(&self.tokens_statements, &mut index, &self.definitions) {
                statements.push(new_statement);
            } else {
                return None;
            }
        }
        self.statements = statements;
        Some(true)
    }

    pub fn parse_procedure(&mut self) -> Option<bool> {
        let mut index = 0;
        let mut definitions: HashMap<String, Rc<ProcedureDefinition>> = HashMap::new();
        let mut tokens_left = Vec::new();
        while index < self.tokens.len() {
            // parse procedure definition
            if &self.tokens[index] == "TO" {
                let new_definition = ProcedureDefinition::new(&self.tokens, &mut index, &definitions)?;
                definitions.insert(new_definition.name.clone(), Rc::new(new_definition));
            } else {
                tokens_left.push(self.tokens[index].clone());
                index += 1;
            }
        }
        self.definitions = definitions;
        self.tokens_statements = tokens_left;
        Some(true)
    }
}
