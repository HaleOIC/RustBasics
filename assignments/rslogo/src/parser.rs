use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
// use crate::statements::Statement;

pub struct Parser {
    lines: Vec<String>,
    tokens: Vec<String>,
    // statements: Vec<Box<dyn Statement>>,
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
            lines: lines,
            tokens: Vec::new(),
            // statements: Vec::new(),
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


// FORWARD [numpixels:f32]
// BACK [numpixels:f32]
// LEFT [numpixels:f32]
// RIGHT [numpixels:f32]
// SETPENCOLOR [colorcode:f32]
// TURN [degrees:f32]
// SETHEADING [degrees:f32]
// SETX [location:f32]
// SETY [location:f32]
    
    // pub fn into_statements(&mut self) {
    //     let mut i = 0;
    //     while i < self.tokens.len() {
    //         let token = &self.tokens[i][..];
    //         match token {
    //             "PENUP" | "PENDOWN" => {
    //                 self.statements.push(Box::new(NoParaStatement {
    //                     command: String::from(token),
    //                 }));
    //             }
    //             "FORWARD" | "BACK" | "LEFT" | "RIGHT" | "TURN" | "SETHEADING" | "SETX" | "SETY" => {
    //                 let arg = &self.tokens[i + 1][1..]
    //                 let arg = self.tokens[i + 1] .parse::<f32>().unwrap();
    //                 self.statements.push(Box::new(UniStatement {
    //                     command: String::from(token),
    //                     arg: Expression::Value(arg as f64),
    //                 }));
    //                 i += 1;
    //             }
    //             _ => {

    //             }
    //         }

    //     }
    // }
}
