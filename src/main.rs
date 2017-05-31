extern crate hound;
extern crate dyon;

use std::io::{self, Write};
use std::fs::File;

use dyon::{error, run};

fn main() {
    error(run("src/scripts/test.dyon"));
}

// fn main() {
//     let sample_rate = 44100;

//     println!("Enter time in seconds: ");
//     let seconds = usize_from_cmd();

//     let mut sound = Vec::with_capacity(sample_rate * seconds);


//     let mut script_module = dyon::Module::new();
//     let match script = dyon::load(script_path, &mut script_module);

//     for x in 0..sample_rate*seconds {
//         sound.push(generate(x));
//     }

//     let mut buffer = File::create("sound").expect("whoops");

//     match buffer.write(&sound) {
//         Ok(x) => {
//             println!("{} bytes written", x)
//         },
//         _ => {
//             panic!("AAAAAAAAA")
//         }
//     }
// }

// fn generate(index: usize) -> i8 {
//     ((index as f64).sin() * (<i8>::max_value() as f64))  as i8
// }

// fn usize_from_cmd() -> usize {
//     let mut num = String::new();

//     io::stdin().read_line(&mut num)
//         .expect("failed to read line");

//     let num: usize = num.trim().parse()
//         .expect("Please type a number!");
//     num
// }