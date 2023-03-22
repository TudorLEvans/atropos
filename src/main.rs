use cpal::{SampleFormat, StreamConfig, Device};
// use dasp_sample::Sample;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{HeapRb};
use ringbuf::consumer::{Consumer};
use std::error::Error;
use std::boxed::Box;
use std::sync::Arc;
use std::{thread};
use std::f32::consts::PI;

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

    let sample_format = supported_config.sample_format();

    let config: StreamConfig = supported_config.into();

    let sample_rate = config.sample_rate.0;

    // create a ring buffer to hold calculated audio data
    let rb = match sample_format {
        SampleFormat::F32 => HeapRb::<f32>::new(16384),
        default => panic!("Unsupported sample format '{sample_format}'")
    };

    let (mut prod, mut cons) = rb.split();

    // calls the thread that writes the ring buffer data to the device
    
    thread::spawn(|| {
        write_to_stream::<f32>(device, cons);
        println!("exited thread")
    });

    let frequency = 200; // classic 440Hz (musical A)
    let tempo = 80;

    // calculate the ring buffer input for the sound wave
    // only write when the buffer is not full
    let mut age = 0;
    let mut s: f32 = 0.0;
    loop {
        if prod.is_full() {
            thread::sleep_ms(1);
        } else {
            age = age + 1;
            s = if age < sample_rate / 6 { (2.0 * PI * frequency as f32 * age as f32 / sample_rate as f32).sin() * ( ( sample_rate as f32 - 6.0 * age as f32 ) / sample_rate as f32)} else {0.0};
            prod.push(s);
            if age >= ( sample_rate * 60 / tempo ) {
                age = 0;
            }
        }
    }
}

// Writes to output device stream
//
// Reads from ringbuf::Consumer in a loop on its own thread and writes to cpal::Stream
fn write_to_stream<T: cpal::SizedSample + Send + std::fmt::Display + 'static>(device: Device, mut cons: Consumer<T, Arc<HeapRb<T>>>) -> Result<bool, Box<dyn Error>> {
    // get supported config
    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    

    // define error closure
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let config: StreamConfig = supported_config.into();
    let sample_rate = config.sample_rate.0;
    let channels = config.channels as usize;

    // return stream based on SampleFormat match
    let stream = device.build_output_stream(&config, 
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {            
            for channel in data.chunks_mut(channels) {
                // closure code for reading from the ring buffer
                for sample in channel.iter_mut() {
                    match cons.pop() {
                        Some(x) => {
                            *sample = x.to_sample::<T>();
                        },
                        None => println!("Empty buffer")
                    }
                }
            }
        },
        err_fn, None).unwrap();

    // play the stream
    stream.play().unwrap();
    thread::sleep_ms(20000);
    Ok(true)
}