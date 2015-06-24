mod generators;
mod filters;
mod file_handling;

use std::path::Path;
use std::io::Write;
use std::io::BufWriter;
use std::fs::OpenOptions;

fn main() {
    println!("Generating a sine wave to sine.wav");

    let mut options = OpenOptions::new();

    options.write(true).create(true);

    let path = Path::new("sine.wav");

    let mut file = match options.open(&path) {
        Ok(file) => file,
        Err(..) => panic!("error opening a file"),
    };

    let mut writer = BufWriter::new(&file);

    writer.write_all(b"pingas\n");
}
