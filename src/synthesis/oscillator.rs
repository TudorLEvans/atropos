use std::f32::consts::PI;
use crate::synthesis::envelope;

pub struct Oscillator {
    tick: u32,
    pub frequency: u32,
    envelope: envelope::Envelope
}

pub fn build_oscillator() -> Oscillator {
    let envelope = envelope::Envelope {
        attack: 40,
        peak_level: 1.0,
        decay: 40,
        sustain_level: 0.1,
        sustain: 100,
        release: 20,
        ttl_ms: 160
    };
    Oscillator {
        frequency: 440,
        tick: 0,
        envelope
    }
}

impl Oscillator {
    pub fn iterate_wave<F32>(&mut self, sample_rate: u32) -> f32 {
        let age = 1000_f64 * f64::from(self.tick) /(f64::from(sample_rate));
        let s = self.envelope.amplitude_modifier(age) * (2.0 * PI * self.frequency as f32 * self.tick as f32 / sample_rate as f32).sin() * ( ( sample_rate as f32 - 8.0 * self.tick as f32 ) / sample_rate as f32);
        self.tick += 1;
        s
    }

    pub fn is_expired(&self, sample_rate: u32) -> bool {
        f64::from(self.envelope.ttl_ms) / 1000.0 <= f64::from(self.tick) / f64::from(sample_rate)
    }
}
