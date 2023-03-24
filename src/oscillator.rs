use std::f32::consts::PI;

pub struct Oscillator {
    pub frequency: u32,
    pub attack: u32,
    pub decay: u32,
    pub sustain: u32,
    pub release: u32,
    ttl_ms: u32, // ttl is when the synth is popped off the buffer -> always attack + decay + sustain + release
    tick: u32,
}

pub fn build_oscillator() -> Oscillator {
    Oscillator {
        frequency: 440,
        attack: 40,
        decay: 0,
        sustain: 100,
        release: 10,
        ttl_ms: 150,
        tick: 0
    }
}

impl Oscillator {
    pub fn iterate_wave<F32>(&mut self, sample_rate: u32) -> f32 {
        let s = self.amplitude_modifier(sample_rate) * (2.0 * PI * self.frequency as f32 * self.tick as f32 / sample_rate as f32).sin() * ( ( sample_rate as f32 - 8.0 * self.tick as f32 ) / sample_rate as f32);
        self.tick += 1;
        s
    }

    fn amplitude_modifier(&self, sample_rate: u32) -> f32 {
        let age = 1000_f64 * f64::from(self.tick) /(f64::from(sample_rate));
        match age {
            age if age <= f64::from(self.attack) => {
                return 1.0 - ((f64::from(self.attack) - age) / f64::from(self.attack)) as f32
            },
            age if f64::from(self.decay) != 0.0 && age < f64::from(self.attack) + f64::from(self.decay) => {
                return 1.0 - 0.99 * (( age - f64::from(self.attack) ) / f64::from(self.decay) ) as f32
            },
            age if f64::from(self.sustain) != 0.0 &&  age < f64::from(self.attack) + f64::from(self.decay) + f64::from(self.sustain) => return 0.01,
            age if f64::from(self.release) != 0.0 &&  age < f64::from(self.attack) + f64::from(self.decay) + f64::from(self.sustain) + f64::from(self.release) => {
                return 0.01 - 0.01 * ( ( age -  (f64::from(self.attack) + f64::from(self.decay) + f64::from(self.sustain))) / f64::from(self.release) ) as f32
            },
            _ => return 0.0
        }
    }

    pub fn is_expired(&self, sample_rate: u32) -> bool {
        f64::from(self.ttl_ms) / 1000.0 <= f64::from(self.tick) / f64::from(sample_rate)
    }
}
