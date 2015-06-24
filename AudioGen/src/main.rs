/*
AudioGen audio generator.
Copyright (C) 2015  Victor Bankowski

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

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
