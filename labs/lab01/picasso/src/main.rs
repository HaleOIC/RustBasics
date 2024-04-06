extern crate bmp;
use bmp::consts;
use std::env;

fn print_img(image_path: &String) {
    let result = bmp::open(image_path);
    let img = match result {
        Ok(imag) => imag,
        Err(e) => {
            println!("Error! {:?}", e);
            return;
        },
    };

    for x in 0..=img.get_width() - 1 {
        for y in 0..=img.get_height() - 1 {
            let pixel = img.get_pixel(y, x);
            match pixel {
                consts::RED => {
                    print!("R ");
                },
                consts::LIME => {
                    print!("G ");
                },
                consts::BLUE => {
                    print!("B ");
                },
                consts::WHITE => {
                    print!("W ");
                },
                _ => {}
            }
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        panic!("missing required command-line arguments");
    }

    for arg in args.iter() {
        println!("===== {} =====", arg);
        print_img(arg);
    }
}
