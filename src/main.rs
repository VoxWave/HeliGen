use cpal::{EventLoop, Format, StreamData, UnknownTypeOutputBuffer};
use std::i16;

fn main() {
    write_noise_wav();
    //run_cpal();
}

fn write_noise_wav() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut sample = 0;
    let mut writer = hound::WavWriter::create("noise2.wav", spec).unwrap();
    writer.write_sample(sample).unwrap();
    for _ in 0 .. 44100*2*60 {
        if rand::random() {
            sample += 1;
        } else {
            sample -= 1;
        }
        writer.write_sample(sample).unwrap();
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
    event_loop.run(move |_stream_id, stream_data| {
        match stream_data {
            StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer)} => {
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
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer)} => {
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
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer)} => {
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
            },
            _ => {},
        }
    });
}
