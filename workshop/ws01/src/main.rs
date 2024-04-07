use bmp::{consts::{BLACK, BLUE, BLUE_VIOLET, GREEN, GREEN_YELLOW, RED, WHITE, WHITE_SMOKE, YELLOW}, Image, Pixel};
use std::path::Path;
use std::io;
const HEIGTH: u32 = 400;
const WIDTH: u32 = 400;
const MAX_ITER: u32 = 100;

fn main() {
    let filename = std::env::args().nth(1).expect("You must provide a path.");
    let operation = std::env::args().nth(2).expect("You must provide an operation.");
    let path = Path::new(&filename);
    
    if path.exists() {
        /* Read the given path and if error occurs, just exit */
        let result = bmp::open(filename.clone());
        match result {
            Ok(_) => {},
            Err(e) => {
                println!("Error! {:?}", e);
                return;
            },
        };
    }
    
    /* Match given operation one by one */
    match operation.as_str() {
        "single" => {
            draw_pixel(&filename);
        },
        "diagonal" => {
            draw_diagonal(&filename);
        },
        "shape-X" => {
            draw_shape_x(&filename);
        },
        "house" => {
            draw_house(&filename);
        },
        "square" => {   
            draw_rectangle(&filename);
        },
        "full-in-square" => {
            draw_square(&filename);
        },
        "rainbow" => {
            draw_rainbow(&filename);
        },
        "Finland" => {
            draw_flag_finland(&filename);
        },
        "Iceland" => {
            draw_flag_iceland(&filename);
        },
        "Aboriginal" => {
            draw_flag_aboriginal(&filename);
        },
        "sine" => {
            draw_sine(&filename);
        },
        "mandelbrot" => {
            draw_mandelbrot(&filename);
        },
        _ => {
            println!("Make sure your operation is valid!");
        },
    }
}

fn draw_rectangle(path: &str) {
    /* Read width and height from users */
    let mut input = String::new();
    println!("Please input width: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read width");
    let width: u32 = input.trim().parse().expect("Failed to convert");
    input.clear();
    println!("Please input height: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read height");
    let height: u32 = input.trim().parse().expect("Failed to convert");
    
    let mut img = Image::new(WIDTH, HEIGTH);
    for x in 100..=100 + width {
        for y in 100..=100 + height {
            img.set_pixel(x, y, BLUE);
        }
    }
    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }  
    }
}

/* Draw a red pixel in the center of canvas */
fn draw_pixel(path: &String) {
    let mut img = Image::new(WIDTH, HEIGTH);
    img.set_pixel(WIDTH / 2, HEIGTH / 2, RED);
    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }
    }
}

/* Draw a diagonal on the figure */
fn draw_diagonal(path: &String) {
    let mut img = Image::new(WIDTH, HEIGTH);
    for x in 0..=img.get_height() - 1 {
        img.set_pixel(x, x, GREEN_YELLOW);
    }
    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }  
    }
}

/* Draw a shape x on the figure */
fn draw_shape_x(path: &String) {
    let mut img = Image::new(WIDTH, HEIGTH);
    for x in 0..=img.get_height() - 1 {
        img.set_pixel(x, x, GREEN);
    }
    for x in 0..=img.get_height() - 1 {
        img.set_pixel(x, img.get_height() - 1 - x, GREEN);
    }
    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }
    }
}



/* Draw a house shape on the figure */
fn draw_house(path: &String) {
    let mut img = Image::new(WIDTH, HEIGTH);

    for x in 50..=200 {
        img.set_pixel( 250 - x, x, WHITE);
        img.set_pixel(150 + x, x, WHITE);
    }
    for y in 50..=350 {
        img.set_pixel(y, 200, WHITE);
    }
    for y in 100..=300 {
        img.set_pixel(y, 350, WHITE);
    }
    for x in 200..=350 {
        img.set_pixel(100, x, WHITE);
        img.set_pixel(300, x, WHITE);
    }

    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }
    }
}


fn draw_square(path: &str) {
    let mut img = Image::new(WIDTH, HEIGTH);
    for x in 100..=300 {
        for y in 100..=300 {
            img.set_pixel(x, y, BLUE_VIOLET);
        }
    }
    for x in 0..=5 {
        for y in 100..=300 {
            img.set_pixel(x + 100, y, WHITE_SMOKE);
            img.set_pixel(300 - x , y, WHITE_SMOKE);
        }
    }
    for y in 0..=5 {
        for x in 100..=300 {
            img.set_pixel(x, y + 100, WHITE_SMOKE);
            img.set_pixel(x, 300 - y, WHITE_SMOKE);
        }
    }
    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }
    }
}

fn draw_rainbow (path: &str) {
    let width = 600;
    let height = 1000;
    let stripe_height = height / 6;

    let mut img = Image::new(width, height);

    for x in 0..width {
        for y in 0..stripe_height {
            img.set_pixel(x, y, bmp::consts::RED); // Red stripe
        }
        for y in stripe_height..(2 * stripe_height) {
            img.set_pixel(x, y, bmp::px!(255, 127, 0)); // Orange stripe
        }
        for y in (2 * stripe_height)..(3 * stripe_height) {
            img.set_pixel(x, y, bmp::consts::YELLOW); // Yellow stripe
        }
        for y in (3 * stripe_height)..(4 * stripe_height) {
            img.set_pixel(x, y, bmp::consts::GREEN); // Green stripe
        }
        for y in (4 * stripe_height)..(5 * stripe_height) {
            img.set_pixel(x, y, bmp::consts::BLUE); // Blue stripe
        }
        for y in (5 * stripe_height)..height {
            img.set_pixel(x, y, bmp::px!(148, 0, 211)); // Violet stripe
        }
    }
    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }
    }
}


fn draw_flag_finland(path: &str) {
    let width = 180;
    let height = 110;
    let mut img = Image::new(width, height);
    
    /* step1: fill in white */
    for x in 0..=width - 1 {
        for y in 0..=height - 1 {
            img.set_pixel(x, y, WHITE);
        }
    }

    /* Step2: fill in dark blue color */ 
    for x in 0..=width - 1{
        for y in 40..=70 {
            img.set_pixel(x, y, bmp::px!(0, 0, 128));
        }
    }
    for x in 50..=80 {
        for y in 0..=height - 1 {
            img.set_pixel(x, y, bmp::px!(0, 0, 128));
        }
    }
    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }
    }


}

fn draw_flag_iceland(path: &str) {
    let width = 200;
    let height = 110;
    let mut img = Image::new(width, height);
    
    /* step1: fill in blue */
    for x in 0..=width - 1 {
        for y in 0..=height - 1 {
            img.set_pixel(x, y, bmp::px!(0, 0, 172));
        }
    }

    /* Step2: fill in white color */ 
    for x in 0..=width - 1{
        for y in 40..=70 {
            img.set_pixel(x, y, WHITE);
        }
    }
    for x in 50..=80 {
        for y in 0..=height - 1 {
            img.set_pixel(x, y, WHITE);
        }
    }

    /* Step2: fill in red color */ 
    for x in 0..=width - 1{
        for y in 48..=62 {
            img.set_pixel(x, y, RED);
        }
    }
    for x in 58..=72 {
        for y in 0..=height - 1 {
            img.set_pixel(x, y, RED);
        }
    }

    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }
    }
}


fn draw_flag_aboriginal(path: &str) {
    let width = 200;
    let height = 110;
    let mut img = Image::new(width, height);
    
    /* step1: fill in black and red background */
    for x in 0..=width - 1 {
        for y in 0..=height / 2 {
            img.set_pixel(x, y, BLACK);
        }
    }
    for x in 0..=width - 1 {
        for y in height / 2..=height - 1 {
            img.set_pixel(x, y, RED);
        }
    }

    /* Step 2 fill in a circle with yellow color */ 
    let circle_center_x = width / 2;
    let circle_center_y = height / 2;
    let circle_radius: i32 = height as i32 / 4; // Adjust the radius as needed

    for x in 0..width {
        for y in 0..height {
            let dx = x as i32 - circle_center_x as i32;
            let dy = y as i32 - circle_center_y as i32;
            if dx * dx + dy * dy <= circle_radius * circle_radius {
                img.set_pixel(x, y, YELLOW);
            }
        }
    }
    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }
    }
}


fn draw_sine(path: &str) {
    let width = 600;
    let height = 100;
    let amplitude = 10.0; // Half the wave height, making the wave 20 pixels high in total
    let frequency = 0.01; // Adjust this for more or fewer waves across the width

    let mut img = Image::new(width, height);

    // Fill the background with white to make the sine wave more visible
    for x in 0..width {
        for y in 0..height {
            img.set_pixel(x, y, bmp::consts::WHITE);
        }
    }

    for x in 0..width {
        // Calculate the sine wave pattern
        let y = ((amplitude * (frequency * x as f32 * 2.0 * std::f32::consts::PI).sin()) + height as f32 / 2.0) as u32;

        // Ensure the y position is within the bounds of the image height
        if y < height {
            // Set the pixel. Using a blue color for the sine wave.
            img.set_pixel(x, y, bmp::px!(0, 0, 255));
        }
    }
    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }
    }

}

fn draw_mandelbrot(path: &str) {
    let mut img = Image::new(800, 600);

    for x in 0..800 {
        for y in 0..600 {
            let iter = mandelbrot_set(x, y);
            let color = if iter < MAX_ITER {
                bmp::Pixel::new((255 * iter / MAX_ITER) as u8, 0, 0)
            } else {
                bmp::Pixel::new(0, 0, 0)
            };
            img.set_pixel(x, y, color);
        }
    }
    let save = img.save(path);
    match save {
        Ok(_) => {},
        Err(error_message) => {
            println!("Failed to save {:?}", error_message);
        }
    }
}

fn mandelbrot_set(x: u32, y: u32) -> u32 {
    let mut zx: f64 = 0.0;
    let mut zy: f64 = 0.0;
    let cx: f64 = 3.0 * x as f64 / 800 as f64 - 2.0;
    let cy: f64 = 2.0 * y as f64 / 600 as f64 - 1.0;

    let mut iter = 0;

    while zx * zx + zy * zy < 4.0 && iter < MAX_ITER {
        let tmp = zx * zx - zy * zy + cx;
        zy = 2.0 * zx * zy + cy;
        zx = tmp;
        iter += 1;
    }

    iter
}