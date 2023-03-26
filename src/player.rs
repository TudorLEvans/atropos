// use std::fs::OpenOptions;
// use std::io::Write;
use std::error::Error;
use std::boxed::Box;
use std::sync::Arc;
use std::{thread};
use std::time::{Duration};

use cpal::{StreamConfig, Device};
use cpal::traits::{DeviceTrait, StreamTrait};
use ringbuf::{HeapRb};
use ringbuf::consumer::{Consumer};

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
    thread::sleep(Duration::from_millis(60000));
    Ok(true)
}