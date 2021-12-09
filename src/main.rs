use cpal::{EventLoop, Format, StreamData, UnknownTypeOutputBuffer};
use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use std::fmt::write;
use std::i16;
use wasmtime::Func;
use wasmtime::Linker;
use wasmtime::Module;
use wasmtime::Store;

fn main() {
    let (doubler_1_sender, doubler_1_receiver) = unbounded();
    let (doubler_2_sender, doubler_2_receiver) = unbounded();
    let (s,r) = unbounded();
    spawn_wasm_thread("generator.wasm".to_string(), vec![], vec![doubler_1_sender]);
    spawn_wasm_thread("doubler.wasm".to_string(), vec![doubler_1_receiver], vec![doubler_2_sender]);
    spawn_wasm_thread("doubler.wasm".to_string(), vec![doubler_2_receiver], vec![s]);
    receiver(r);
}

fn spawn_wasm_thread(module_name: String, inputs: Vec<Receiver<f64>>, outputs: Vec<Sender<f64>>) {
    std::thread::spawn(move || {
        let store = Store::default();
        let mut linker = Linker::new(&store);
        linker.define("heligen", "output", Func::wrap(&store, move |channel: u64, output: f64| {
            match outputs.get(channel as usize) {
                Some(channel) => channel.send(output).unwrap(),
                None => println!("value {} sent to unconnected channel number {}", output, channel),
            };
        })).unwrap();
        linker.define("heligen", "input", Func::wrap(&store, move |channel: u64| -> f64 {
            match inputs.get(channel as usize) {
                Some(channel) => channel.recv().unwrap(), // Figure out how to shut down cleanly when a channel closes.
                None => 0.,
            }
        })).unwrap();
        let module_file = std::fs::read(module_name).unwrap();
        let module = Module::new(store.engine(), module_file).unwrap();
        let instance = linker.instantiate(&module).unwrap();
        let start_func = instance.get_func("heligen_start").unwrap().get0::<()>().unwrap();
        start_func().unwrap();
    });
}

fn receiver(r: Receiver<f64>) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut i = 0;
    let mut writer = hound::WavWriter::create("test.wav", spec).unwrap();
    while let Ok(sample) = r.recv() {
        if i >= 44100*10 {
            break;
        }
        writer.write_sample(sample as f32).unwrap();
        i += 1;
    }
}

fn write_noise_wav() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };
    let mut sample: i16 = 0;
    let mut vel = 0;
    let mut writer = hound::WavWriter::create("noise2.wav", spec).unwrap();
    writer.write_sample(sample).unwrap();
    for _ in 0..44100 * 2 * 60 {
        if rand::random() {
            vel += 1;
        } else {
            vel -= 1;
        }
        sample += vel;
        match writer.write_sample(sample) {
            Ok(_) => {}
            Err(_) => {
                println!("{}, {}", sample, vel);
                panic!();
            }
        };
    }
}

fn run_cpal() {
    let event_loop = EventLoop::new();
    let device = cpal::default_output_device().expect("No device available");
    let format = device.default_output_format().expect("no default format");
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);
    let mut flippo = true;
    let mut counter = 0;
    event_loop.run(move |_stream_id, stream_data| match stream_data {
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::F32(mut buffer),
        } => {
            println!("f32");
            for elem in buffer.iter_mut() {
                counter += 1;
                if counter > 500 {
                    counter = 0;
                    flippo = !flippo;
                }
                if flippo {
                    *elem = 1.0;
                } else {
                    *elem = -1.0;
                }
            }
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::I16(mut buffer),
        } => {
            println!("i16");
            for elem in buffer.iter_mut() {
                counter += 1;
                if counter > 500 {
                    counter = 0;
                    flippo = !flippo;
                }
                if flippo {
                    *elem = i16::max_value();
                } else {
                    *elem = i16::min_value();
                }
            }
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::U16(mut buffer),
        } => {
            println!("u16");
            for elem in buffer.iter_mut() {
                counter += 1;
                if counter > 500 {
                    counter = 0;
                    flippo = !flippo;
                }
                if flippo {
                    *elem = u16::max_value();
                } else {
                    *elem = u16::min_value();
                }
            }
        }
        _ => {}
    });
}
