use std::f64::consts::PI;

use config::SAMPLE_RATE;

const SAW: &'static [f64; 10] = &[
    1.0,  2.0,  3.0,  4.0,  5.0,
    6.0,  7.0,  8.0,  9.0,  10.0,
];

const SQUARE: &'static [f64; 10] = &[
    1.0,  3.0,  5.0,  7.0,  9.0,
    11.0, 13.0, 15.0, 17.0, 19.0,
];

const TAU: f64 = PI * 2.0;

#[derive(Clone, Copy)]
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
        let next = Some(self.amplitude * (TAU * self.frequency * self.time).sin());
        self.time += 1.0 / SAMPLE_RATE;
        next
    }
}

#[derive(Clone, Copy)]
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
        let next = Some(SAW.iter().map(|n| self.amplitude * (TAU * n * self.frequency * self.time).sin()).sum::<f64>() / 10.0);
        self.time += 1.0 / SAMPLE_RATE;
        next
    }
}

#[derive(Clone, Copy)]
pub struct Square {
    amplitude: f64,
    frequency: f64,
    time: f64,
}

impl Square {
    pub fn new(amplitude: f64, frequency: f64) -> Self {
        Square { amplitude, frequency, time: 0.0 }
    }
}

impl Iterator for Square {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let next = Some(SQUARE.iter().map(|n| self.amplitude * (TAU * n * self.frequency * self.time).sin()).sum::<f64>() / 10.0);
        self.time += 1.0 / SAMPLE_RATE;
        next
    }
}

#[derive(Copy, Clone)]
pub struct Silence {}

impl Iterator for Silence {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        Some(0.0)
    }
}
