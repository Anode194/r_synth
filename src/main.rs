use cpal::{Data, Sample, SampleFormat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    println!("hello world!");
    use cpal::traits::HostTrait;
                                    //get default audio host which give you access to the available
                                    //devices
    let host = cpal::default_host();
                                    //create your default device.
    let device = host.default_output_device().expect("no output device available");
                                    //gets the range of supported output configurations
    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs device may be disconnected");
                                    //saves the supported output config with the maximum sample
                                    //rate.
    let err_fn = |err| eprintln!(" an error occured on the output audio stream: {}", err);
    let supported_config = supported_configs_range.next()
        .expect("No supported config??")
        .with_max_sample_rate();
                                    //get the supported sample format (f32, i16,u16) from the
                                    //supported config.
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
                                // create audio stream.
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, write_sin_wave::<f32>, err_fn),
        SampleFormat::I16 => device.build_output_stream(&config, write_sin_wave::<i16>, err_fn),
        SampleFormat::U16 => device.build_output_stream(&config, write_sin_wave::<u16>, err_fn),
    }.unwrap();
}

fn write_sin_wave<T:Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {

        *sample = Sample::from(&0.0);
    }
}
