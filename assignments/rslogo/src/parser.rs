use crate::statements::{parse_statement, Statement};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Parser {
    lines: Vec<String>,
    tokens: Vec<String>,
    statements: Vec<Box<dyn Statement>>,
}

impl Parser {
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
            statements: Vec::new(),
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

    pub fn show_tokens(&self) {
        for token in &self.tokens {
            println!("{}", token);
        }
    }

    pub fn show_statements(&self) -> &Vec<Box<dyn Statement>> {
        return &self.statements;
    }

    pub fn into_statements(&mut self) -> Option<bool> {
        let mut index = 0;
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        while index < self.tokens.len() {
            let new_statement = parse_statement(&self.tokens, &mut index)?;
            statements.push(new_statement);
        }
        self.statements = statements;
        Some(true)
    }
}
