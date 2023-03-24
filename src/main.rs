use std::{thread};
use std::collections::VecDeque;
use std::time::{Duration};

use cpal::{SampleFormat, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait};
use ringbuf::{HeapRb};

mod oscillator;
mod player;


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
        sample_format => panic!("Unsupported sample format '{sample_format}'")
    };

    let (mut prod, cons) = rb.split();

    // calls the thread that writes the ring buffer data to the device
    thread::spawn(|| {
        player::write_to_stream::<f32>(device, cons).unwrap();
        println!("exited thread");
    });

    let tempo = 80;
    let bar_length = 4;

    // create a (non-threadable) buffer for storing the FIFO list of notes currently playing
    let mut notes = VecDeque::new();

    // calculate the ring buffer input for the sound wave
    // only write when the buffer is not full
    let mut s: f32;
    let mut age = 0;
    loop {
        if prod.is_full() {
            thread::sleep(Duration::from_millis(1));
        } else {
            s = 0.0;
            if age % ( sample_rate * 60 / tempo) == 0 {
                notes.push_back(oscillator::build_oscillator());
            };
            match notes.front() {
                Some(x) => {
                    if x.is_expired(sample_rate) {
                        notes.pop_front();
                    };
                },
                None => (),
            }
            for note in notes.iter_mut() {
                s = s + note.iterate_wave::<f32>(sample_rate);
            }
            // dangerous - FIX!
            if notes.len() != 0 {
                s = s / notes.len() as f32;
            } else {
                s = 0.0;
            }

            prod.push(s).unwrap();
            age = age + 1;

            // reset age after each bar completes
            if age == sample_rate * bar_length * (60 / tempo) {
                age = 0
            }
        }
    }
}
