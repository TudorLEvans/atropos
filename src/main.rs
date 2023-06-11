use std::{thread};
use std::collections::VecDeque;
use std::time::{Duration};

use cpal::{SampleFormat, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait};
use ringbuf::{HeapRb};

use clap::{Parser};

mod synthesis;
mod player;
mod cli;


fn main() {
    let cli = cli::Arguments::parse();

    match cli.cmd {
        cli::Commands::Met(met) => {
            process_met(met)
        }
    }
}

fn process_met(met: cli::Metronome) {

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
        SampleFormat::F32 => HeapRb::<f32>::new(8192),
        sample_format => panic!("Unsupported sample format '{sample_format}'")
    };

    let (mut prod, cons) = rb.split();

    // calls the thread that writes the ring buffer data to the device
    thread::spawn(|| {
        player::write_to_stream::<f32>(device, cons).unwrap();
        println!("exited thread");
    });

    // create a (non-threadable) buffer for storing the FIFO list of notes currently playing
    let mut notes = VecDeque::new();

    let sequence = player::generate_sequence(sample_rate, &met);
    println!("{:?}", sequence);

    // calculate the ring buffer input for the sound wave
    // only write when the buffer is not full
    let mut s: f32;
    let mut age: u32 = 0;
    let mut index: usize = 0;
    let mut reset: bool = false;
    loop {
        if prod.is_full() {
            thread::sleep(Duration::from_millis(1));
        } else {
            s = 0.0;

            // check which type of note to play and push on stack
            if sequence[index].age == age {
                if sequence[index].note == 0 && met.bell {
                    notes.push_back(synthesis::oscillator::build_oscillator(700));
                } else if sequence[index].note < 2 && sequence[index].note >= 0  {
                    notes.push_back(synthesis::oscillator::build_oscillator(600));
                } else if sequence[index].note == 2 {
                    notes.push_back(synthesis::oscillator::build_oscillator(440));
                } else {
                    reset = true;
                }
                index = ( index + 1 ) % sequence.len();
            }

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

            if notes.len() != 0 {
                s = s / notes.len() as f32;
            } else {
                s = 0.0;
            }

            prod.push(s).unwrap();
            age = age + 1;

            // reset age after each bar completes
            if reset {
                age = 0;
                reset = false;
            }
        }
    }
}