use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use std::fmt::write;
use std::i16;
use crate::wasm::spawn_wasm_thread;
use crate::output::output_to_file;

use wasmtime::Store;

mod output;
mod module_setup;
mod wasm;

fn main() {
    let (doubler_1_sender, doubler_1_receiver) = unbounded();
    let (doubler_2_sender, doubler_2_receiver) = unbounded();
    let (s, r) = unbounded();
    spawn_wasm_thread("generator.wasm".to_string(), vec![], vec![doubler_1_sender]);
    spawn_wasm_thread(
        "doubler.wasm".to_string(),
        vec![doubler_1_receiver],
        vec![doubler_2_sender],
    );
    spawn_wasm_thread(
        "doubler.wasm".to_string(),
        vec![doubler_2_receiver],
        vec![s],
    );
    output_to_file(r, "test.wav");}
