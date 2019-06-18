use cpal::{EventLoop, Format, StreamData, UnknownTypeOutputBuffer};

fn main() {
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
