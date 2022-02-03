use cpal::{EventLoop, Format, StreamData, UnknownTypeOutputBuffer};

use crossbeam::channel::Receiver;

pub fn output_to_file(r: Receiver<f64>, filename: &str) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut i = 0;
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    while let Ok(sample) = r.recv() {
        if i >= 44100 * 10 {
            break;
        }
        writer.write_sample(sample as f32).unwrap();
        i += 1;
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