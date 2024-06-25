use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::statements::{NoParaStatement, Statement, UniStatement};

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

    pub fn show_statements(&self) -> &Vec<Box<dyn Statement>>{
        return &self.statements;
    }

    pub fn into_statements(&mut self) -> Option<bool> {
        let mut index = 0;
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        while index < self.tokens.len() {
            let cur_token = &self.tokens[index][..];
            match cur_token {
                "PENUP" | "PENDOWN" => {
                    let (new_statement, new_index) = NoParaStatement::new(&self.tokens, index)?;
                    statements.push(Box::new(new_statement));
                    index = new_index + 1;
                }
                "FORWARD" | "BACK" | "LEFT" | "RIGHT" | "SETPENCOLOR" | "TURN" | "SETHEADING" | "SETX" | "SETY" => {
                    let (new_statement, new_index) = UniStatement::new(&self.tokens, index)?;
                    statements.push(Box::new(new_statement));
                    index = new_index + 1;
                }
                _ => {
                    eprintln!("Unknown command: {}", cur_token);
                    return None;
                }   
            }
        }
        self.statements = statements;
        Some(true)

    }
}
