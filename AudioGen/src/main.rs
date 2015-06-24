mod generators;
mod filters;
mod file_handling;

use std::path::Path;
use std::io::BufWriter;
use std::fs::OpenOptions;

fn main() {
    println!("Generating a sine wave to sine.wav");

    let mut options = OpenOptions::new();

    options.write(true).append(true);

    let path = Path::new("sine.wav");

    let file = match options.open(&path) {
        Ok(file) => file,
        Err(..) => panic!("error opening a file"),
    };

    let mut writer = BufWriter::new(&file);
}
