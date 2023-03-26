use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::process;

use image::{ImageError, DynamicImage};
use image::codecs::jpeg::JpegEncoder;
use image::codecs::webp::WebPDecoder;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_file.webp> <output_file.jpg>", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    if let Err(e) = convert_webp_to_jpg(input_file, output_file) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    if let Err(e) = delete_orig(Path::new(input_file)) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn delete_orig(path: &Path) -> std::io::Result<()> {
    std::fs::remove_file(path)
}

fn convert_webp_to_jpg(input_file: &str, output_file: &str) -> Result<(), ImageError> {
    let input_path = Path::new(input_file);
    let output_path = Path::new(output_file);

    let input = File::open(input_path)?;
    let decoder = WebPDecoder::new(input)?;
    let image = DynamicImage::from_decoder(decoder)?;

    let output = File::create(output_path)?;
    let output_writer = BufWriter::new(output);
    let mut encoder = JpegEncoder::new_with_quality(output_writer, 80);

    encoder.encode_image(&image)?;

    Ok(())
}
