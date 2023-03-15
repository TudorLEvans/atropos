use cpal::{SampleFormat};
use dasp_sample::Sample;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

struct Metronome {
    bar_length: u8,
    sub_divisions: u8,
    tempo: i16,
    use_bell: bool,
    use_beat: bool
}

fn main() {
    // define the host
    let host = cpal::default_host();

    // obtain the default device
    let device = host.default_output_device().expect("no output device available");

    println!("{}", device.name().unwrap());

    // get supported config
    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    // define error closure
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    // define format of sample
    let sample_format = supported_config.sample_format();

    let config = supported_config.into();

    // return stream based on SampleFormat match
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, write_square, err_fn, None),
        sample_format => panic!("Unsupported sample format '{sample_format}'")
    }.unwrap();

    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(10));
}


fn write_square(data: &mut [f32], _: &cpal::OutputCallbackInfo) {
    let mut counter = 0;
    for sample in data.iter_mut() {
        let s = if (counter / 20) % 2 == 0 { &1.0 } else { &0.0 };
        counter = counter + 1;
        *sample = s.to_sample::<f32>()
    }
}
