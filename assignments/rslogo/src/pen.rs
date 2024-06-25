use unsvg::{Color, COLORS};

pub struct Pen {
    color: Color,
    pos_x: f32,
    pos_y: f32,
    degree: i32,
    on_image: bool,
    color_number: usize,
}

impl Pen {
    /// Create a new pen with default values.
    /// ```
    /// use rslogo::pen::Pen;
    /// let pen = Pen::new(200, 200);
    /// assert_eq!(pen.get_x(), 100.0);
    /// assert_eq!(pen.get_y(), 100.0);
    /// ```
    pub fn new(height: u32, weight: u32) -> Self {
        Self {
            color: Color::white(),
            pos_x: height as f32 / 2.0,
            pos_y: weight as f32 / 2.0,
            degree: 0,
            on_image: false,
            color_number: 7,
        }
    }

    /// Setter function for the color of the pen.
    /// ```
    /// use rslogo::pen::Pen;
    /// let mut pen = Pen::new(200, 200);
    /// assert_eq!(pen.set_color(1), true);
    /// assert_eq!(pen.set_color(16), false);
    /// assert_eq!(pen.set_color(255), false);
    /// assert_eq!(pen.get_color_number(), 1);
    /// ```
    pub fn set_color(&mut self, code: usize) -> bool {
        if code < 16 {
            self.color = COLORS[code];
            self.color_number = code;
            true
        } else {
            false
        }
    }
    pub fn get_color(&self) -> Color {
        return self.color;
    }
    pub fn get_color_number(&self) -> usize {
        return self.color_number;
    }

    /// setter and getter function for the cordinate of the pen
    /// ```
    /// use rslogo::pen::Pen;
    /// let mut pen = Pen::new(200, 200);
    /// pen.set_x(10.0);
    /// pen.set_y(20.0);
    /// assert_eq!(pen.get_x(), 10.0);
    /// assert_eq!(pen.get_y(), 20.0);
    /// ```
    pub fn set_x(&mut self, x: f32) {
        self.pos_x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.pos_y = y;
    }
    pub fn get_x(&self) -> f32 {
        return self.pos_x;
    }
    pub fn get_y(&self) -> f32 {
        return self.pos_y;
    }

    /// setter and getter function for the degree of the pen
    /// ```
    /// use rslogo::pen::Pen;
    /// let mut pen = Pen::new(200, 200);
    /// pen.set_degree(90);
    /// assert_eq!(pen.get_degree(), 90);
    /// pen.set_degree(180);
    /// assert_eq!(pen.get_degree(), 180);
    /// ```
    pub fn set_degree(&mut self, degree: i32) {
        self.degree = degree;
    }
    pub fn get_degree(&self) -> i32 {
        return self.degree;
    }
    pub fn turn_degree(&mut self, degree: i32) {
        self.degree += degree;
    }

    /// setter and getter function for the on_image status of the pen
    /// ```
    /// use rslogo::pen::Pen;
    /// let mut pen = Pen::new(200, 200);
    /// pen.set_on_image(true);
    /// assert_eq!(pen.get_on_image(), true);
    /// pen.set_on_image(false);
    /// assert_eq!(pen.get_on_image(), false);
    /// ```
    pub fn set_on_image(&mut self, status: bool) {
        self.on_image = status;
    }
    pub fn get_on_image(&self) -> bool {
        return self.on_image;
    }
}
