use std::default::Default;

pub struct Envelope {
    pub attack: u32,
    pub peak_level: f32,
    pub decay: u32,
    pub sustain_level: f32,
    pub sustain: u32,
    pub release: u32,
    pub ttl_ms: u32, // ttl is when the synth is popped off the buffer -> always attack + decay + sustain + release
}

impl Envelope {
    pub fn amplitude_modifier(&self, age: f64) -> f32 {
        match age {
            age if age <= f64::from(self.attack) => {
                return self.peak_level - ((f64::from(self.attack) - age) / f64::from(self.attack)) as f32
            },
            age if f64::from(self.decay) != 0.0 && age < f64::from(self.attack) + f64::from(self.decay) => {
                return self.peak_level - (self.peak_level - self.sustain_level) * (( age - f64::from(self.attack) ) / f64::from(self.decay) ) as f32
            },
            age if f64::from(self.sustain) != 0.0 &&  age < f64::from(self.attack) + f64::from(self.decay) + f64::from(self.sustain) => return self.sustain_level,
            age if f64::from(self.release) != 0.0 &&  age < f64::from(self.attack) + f64::from(self.decay) + f64::from(self.sustain) + f64::from(self.release) => {
                return self.sustain_level * ( 1.0 - ( ( age -  (f64::from(self.attack) + f64::from(self.decay) + f64::from(self.sustain))) / f64::from(self.release) ) as f32  )
            },
            _ => return 0.0
        }
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope { attack: 40, peak_level: 1.0, decay: 100, sustain_level: 0.1, sustain: 0, release: 10, ttl_ms: 150}
    }
}
