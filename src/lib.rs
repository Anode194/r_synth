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
                *elem =div ;
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
#[allow(dead_code)]
pub struct WaveTable {
   pub wave: Vec<f64>
}
#[allow(dead_code)]
impl WaveTable {
    pub fn new() -> WaveTable {
       WaveTable { wave: Vec::new() } 
    }

#[allow(dead_code)]
    pub fn sin_wave(amplitude: f64, period: i16) ->  WaveTable {
        let mut sin = WaveTable::new();
        for elem in 0..period {
            let table_size = elem as f64;
            let mut value = amplitude * table_size.sin();
            //value = value * 50.0;
            sin.wave.push(value);
        }
        sin
    }
//    fn square_wave<T>(amplitude: period: i16) -> WaveTable {

 //   }
    //pub fn ramp_up(height: f32, period:f32) {}
    //pub fn ramp_down(height: f32, period:i16) {}
}
    //pub fn noise() {}
        
pub fn print_waveTable(table: WaveTable) {
    for elem in table.wave.iter() {
        println!("{}",elem);
    }
}
