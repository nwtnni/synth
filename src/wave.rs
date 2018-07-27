use std::f64::consts::PI;

use config::SAMPLE_RATE;

pub struct Sine {
    amplitude: f64,
    frequency: f64,
    time: f64,
}

impl Sine {
    pub fn new(amplitude: f64, frequency: f64) -> Self {
        Sine { amplitude, frequency, time: 0.0 }
    }
}

impl Iterator for Sine {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let next = Some(self.amplitude * (2.0 * PI * self.frequency * self.time).sin());
        self.time += 1.0 / SAMPLE_RATE;
        next
    }
}

pub struct Sawtooth {
    amplitude: f64,
    frequency: f64,
    time: f64,
}

impl Sawtooth {
    pub fn new(amplitude: f64, frequency: f64) -> Self {
        Sawtooth { amplitude, frequency, time: 0.0 }
    }
}

impl Iterator for Sawtooth {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let next = Some((1..50).map(|n| self.amplitude * (n as f64 * 2.0 * PI * self.frequency * self.time).sin()).sum::<f64>() / 50.0);
        self.time += 1.0 / SAMPLE_RATE;
        next
    }
}
