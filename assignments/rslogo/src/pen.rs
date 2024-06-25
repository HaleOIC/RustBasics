pub struct Pen {
    color: String,
    pos_x: f32,
    pos_y: f32,
    degree: f32,
    on_image: bool,
}

impl Pen {
    /// Create a new pen with default values.
    /// ```
    /// use rslogo::pen::Pen;
    /// let pen = Pen::new();
    /// ```
    pub fn new() -> Self {
        Self {
            color: String::from("white"),
            pos_x: 0.0,
            pos_y: 0.0,
            degree: 0.0,
            on_image: false,
        }
    }

    /// Setter and getter function for the color of the pen.
    /// ```
    /// use rslogo::pen::Pen;
    /// let mut pen = Pen::new();
    /// pen.set_color("red");
    /// assert_eq!(pen.get_color(), "red");
    /// ```
    pub fn set_color(&mut self, color: &str) {
        self.color = String::from(color);
    }
    pub fn get_color(&self) -> &str {
        &self.color
    }


    /// setter and getter function for the cordinate of the pen
    /// ```
    /// use rslogo::pen::Pen;
    /// let mut pen = Pen::new();
    /// pen.set_x(10.0);
    /// pen.set_y(20.0);
    /// assert_eq!(pen.get_x(), 10.0);
    /// assert_eq!(pen.get_y(), 20.0);
    /// ```
    pub fn set_x(&mut self, x:f32) {
        self.pos_x = x;
    }
    pub fn set_y(&mut self, y:f32) {
        self.pos_y = y;
    }
    pub fn get_x(&self) -> f32{
        return self.pos_x;
    }
    pub fn get_y(&self) -> f32{
        return self.pos_y;
    }

    /// setter and getter function for the degree of the pen
    /// ```
    /// use rslogo::pen::Pen;
    /// let mut pen = Pen::new();
    /// pen.set_degree(90.0);
    /// assert_eq!(pen.get_degree(), 90.0);
    /// pen.set_degree(180.0);
    /// assert_eq!(pen.get_degree(), 180.0);
    /// ```
    pub fn set_degree(&mut self, degree: f32) {
        self.degree = degree;
    }
    pub fn get_degree(&self) -> f32 {
        return self.degree;
    }

    /// setter and getter function for the on_image status of the pen
    /// ```
    /// use rslogo::pen::Pen;
    /// let mut pen = Pen::new();
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
