pub struct Pen {
    color: String,
    pos_x: f32,
    pos_y: f32,
    degree: f32,
    on_image: bool,
    image
}

impl Pen {
    /// Create a new pen with default values.
    /// ```
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

    // pub fn parse_command(cmd: &str) {
    //     match
    // }
}
