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
use std::path::Path;
use std::io::Write;
use std::io::BufWriter;
use std::fs::OpenOptions;

pub struct WavFile {
    header: WavFileHeader,
    path: String,
    data: Vec<i16>,
    length: u32,
    channels: u16,
    sample_rate: u32,
    bits_per_sample: u8,
}

struct WavFileHeader {

}

impl WavFile {
    pub fn new(path: String, channels: u16, sample_rate: u32, bits_per_sample: u8) -> WavFile {
        WavFile {
            path: path,
            data: Vec::new(),
            length: 0,
            channels: channels,
            sample_rate: sample_rate,
            bits_per_sample: bits_per_sample,
        }
    }

    pub fn export(&self) {
        let mut options = OpenOptions::new();

        options.write(true).create(true).truncate(true);

        let path = self.path;

        let file = match options.open(&path) {
            Ok(file) => file,
            Err(..) => panic!("error opening a file"),
        };

        let mut writer = BufWriter::new(&file);

        writer.write_all(b"RIFF");
        writer.write_all(b"test");
    }
}
