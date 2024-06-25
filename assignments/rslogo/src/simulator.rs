use crate::{parser, pen};
use unsvg::Image;

pub struct Simulator<'a, 'b> {
    parser: &'a mut parser::Parser,
    pen: pen::Pen,
    image: &'b mut Image,

}

impl<'a, 'b> Simulator<'a, 'b> {
    pub fn new(parser: &'a mut parser::Parser, image: &'b mut Image) -> Self {
        Self {
            parser,
            pen: pen::Pen::new(),
            image,
        }
    }

    pub fn simulate(&mut self) {
        self.parser.show_tokens();
    }
}