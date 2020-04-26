extern crate cpal;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeOutputBuffer};
use rand::prelude::*;

pub fn main_loop() {
    let host = cpal::default_host();
    let event_loop = host.event_loop();
    let device = host.default_output_device()
        .expect("no output device available");
    let mut supported_formats_range = device.supported_output_formats()
        .expect("error while querying for audio formats");
    let format = supported_formats_range.next()
        .expect("no supported format")
        .with_max_sample_rate();
    let _stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.run(move | _stream_id, stream_result| {
            let stream_data = match stream_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("an error occurred on stream {:?}: {}", _stream_id, err);
            return;
        }
        //_ => return,
    };
            let mut rng = rand::thread_rng();

    match stream_data {
        StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
            let  div: u16 = rng.gen_range(1,u16::max_value());
            for elem in buffer.iter_mut() {
                *elem = u16::max_value() / div;
            }
        },
        StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
            for elem in buffer.iter_mut() {
                let div: i16 = rng.gen_range(1,i16::max_value());
                *elem = div;
            }
        },
        StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
            for elem in buffer.iter_mut() {
                let  div: f32 = rng.gen_range(1.0,std::f32::MAX);
                *elem = std::f32::MAX / div;
            }
        },
        _ => (),
    }
    });
}
//TODO: functions for generating various waves. these should be generic so they can be used with
//any type of stream_data
#[allow(dead_code)]
fn sin_wave() -> i16 {

2
}
#[allow(dead_code)]
fn square_wave<T>(pulse_width: T) -> T {
   pulse_width 
}
#[allow(dead_code)]
fn ramp_up() {}
#[allow(dead_code)]
fn ramp_down() {}
#[allow(dead_code)]
fn noise() {}
