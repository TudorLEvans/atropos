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
use std::collections::VecDeque;

struct Metronome {
    bar_length: u8,
    sub_divisions: u8,
    tempo: i16,
    use_bell: bool,
    use_beat: bool
}

struct Synth {
    frequency: u16,
    ttl_ms: u16,
    tick: u32,
}

fn build_synth() -> Synth {
    Synth {
        frequency: 440,
        ttl_ms: 100,
        tick: 0
    }
}

impl Synth {
    fn iterate_wave<T>(&mut self, sample_rate: i32) -> T {
        s = (2.0 * PI * frequency as f32 * self.tick as f32 / sample_rate as f32).sin() * ( ( sample_rate as f32 - 8.0 * self.tick as f32 ) / sample_rate as f32);
        self.tick += 1;
        s
    }
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
        println!("exited thread");
    });

    let tempo = 80;
    let bar_length = 4;

    // create a (non-threadable) buffer for storing the FIFO list of notes currently playing
    let notes = VecDeque::new<Synth>();

    // calculate the ring buffer input for the sound wave
    // only write when the buffer is not full
    let mut s: f32 = 0.0;
    let mut age = 0;
    loop {
        if prod.is_full() {
            thread::sleep_ms(1);
        } else {
            if  age % ( sample_rate * 60 / tempo) == 0 {
                notes.push_back(build_synth());
            };
            match notes.front() {
                Some(x) => {
                    if ( 1000 * x.tick / sample_rate >= x.ttl ) {
                        notes.pop_front();
                    };
                },
                None => (),
            }
            for note in notes.iter_mut() {
                s = note.iterate_wave<f32>(note, sample_rate);
                prod.push(s);
                age = age + 1;
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