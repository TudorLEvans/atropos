use std::error::Error;
use std::boxed::Box;
use std::sync::Arc;

use cpal::{StreamConfig, Device};
use cpal::traits::{DeviceTrait, StreamTrait};
use ringbuf::{HeapRb};
use ringbuf::consumer::{Consumer};

use crate::cli;

#[derive(Debug)]
pub struct SequenceNote {
    pub age: u32,
    pub note: i8,
}

// Creates the sequence of notes and correct sample no. at which to play the note
// this fixes an earlier duplication error and allows more complex sampling logic
pub fn generate_sequence(sample_rate: u32, met: &cli::Metronome) -> Vec<SequenceNote> {
    let mut sequence = Vec::new();
    for n in 0..=met.sub_divisions*met.bar_length {
        let play = match met.seq.get(n as usize) {
            Some(m) if *m == "1" => true,
            Some(m) if *m == "0" => false,
            None => true,
            Some(m) => panic!("Invalid value in sequencer argument: {}", m)
        };
        let age: u32 =  n * 60 * sample_rate / ( met.tempo * met.sub_divisions );
        if n == met.sub_divisions*met.bar_length {
            let note = SequenceNote { age, note: -1 };
            sequence.push(note);
        } else if play {
            // add bell note
            if age == 0 {
                let note = SequenceNote { age, note: 0 };
                sequence.push(note);
            // add normal note
            } else if n % met.sub_divisions == 0 {
                let note = SequenceNote { age, note: 1 };
                sequence.push(note);
            // add subdivisions
            } else {
                let note = SequenceNote { age, note: 2 };
                sequence.push(note);
            }
        }
    }
    sequence
}

// Writes to output device stream
//
// Reads from ringbuf::Consumer in a loop on its own thread and writes to cpal::Stream
pub fn write_to_stream<T: cpal::SizedSample + Send + std::fmt::Display + 'static>(device: Device, mut cons: Consumer<T, Arc<HeapRb<T>>>) -> Result<bool, Box<dyn Error>> {
    // get supported config
    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    
    // define error closure
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let config: StreamConfig = supported_config.into();
    let channels = config.channels as usize;

    // return stream based on SampleFormat match
    let stream = device.build_output_stream(&config, 
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            // closure code for reading from the ring buffer
            // get the volume, iterate through channels, write to output
            for channel in data.chunks_mut(channels) {
                match cons.pop() {
                    Some(x) => {
                        for sample in channel.iter_mut() {
                            *sample = x.to_sample::<T>();
                        }
                    },
                    None => {}
                }
            }
        },
        err_fn, None).unwrap();

    // play the stream
    stream.play().unwrap();

    // control loop so thread runs until main exits
    loop {
        // will add more control logic using channels here
    }
    Ok(true)
}
