// Ignore this file for now - will be used to form more complex synthesis in future
use std::collections::Vec;

mod oscillator;
mod envelope;


pub struct AdditiveSynth {
    tick: u32,
    oscillators: Vec<oscillator::Oscillator>
}