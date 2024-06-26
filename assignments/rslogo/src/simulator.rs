use std::collections::HashMap;

use crate::{expression::Outcome, parser, pen};
use unsvg::Image;

pub struct Simulator<'a, 'b> {
    parser: &'a mut parser::Parser,
    pen: pen::Pen,
    image: &'b mut Image,
    values: HashMap<String, Outcome>,
}

impl<'a, 'b> Simulator<'a, 'b> {
    pub fn new(parser: &'a mut parser::Parser, image: &'b mut Image) -> Self {
        let (h, w) = image.get_dimensions();
        Self {
            parser,
            pen: pen::Pen::new(h, w),
            image,
            values: HashMap::new()
        }
    }

    pub fn simulate(&mut self) -> Result<(), String>{
        // parse tokens into procedure
        if self.parser.parse_procedure().is_none() {
            return Err("Failed to parse procedure".to_string());
        }
        
        // parse left tokens into statements
        if self.parser.into_statements().is_none() {
            return Err("Failed to parse statements".to_string());
        }

        for statement in self.parser.show_statements() {
            if !statement.execute(&mut self.values, &mut self.pen, self.image) {
                return Err("Failed to execute statement".to_string());
            }
        }
        Ok(())
    }
}