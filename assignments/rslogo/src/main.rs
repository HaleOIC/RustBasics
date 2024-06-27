use clap::Parser;
use rslogo::{parser, simulator};
use unsvg::Image;

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments and fetch corresponding string
    let file_path = args
        .file_path
        .to_str()
        .expect("Can not figure out file path");
    let image_path = args
        .image_path
        .to_str()
        .expect("Can not figure out image path");
    let height = args.height;
    let width = args.width;

    // Instantiate an output image
    let mut image = Image::new(width, height);
    match simulation(file_path, image) {
        Some(img) => {
            image = img;
        }
        _ => {
            return Err(());
        }
    }

    // save the image according to its extension
    match args.image_path.extension().map(|s| s.to_str()).flatten() {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    };

    Ok(())
}

// start simulation and get result
fn simulation(file_path: &str, mut image: Image) -> Option<Image> {
    // parse the given file_path
    let mut parser = parser::Parser::parse_file(file_path)
        .map_err(|s| {
            eprintln!("{}", s);
            s
        })
        .ok()?;

    // convert lines into tokens
    parser.into_tokens();

    // new an object for simulation
    let mut simulator = simulator::Simulator::new(&mut parser, &mut image);
    if simulator.simulate().is_err() {
        return None;
    }
    Some(image)
}
