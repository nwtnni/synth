use std::f64::consts::PI;

use config::SAMPLE_RATE;

const SILENCE: &'static [f64; 1] = &[
    0.0,
];

const SINE: &'static [f64; 1] = &[
    1.0,
];

const SAWTOOTH: &'static [f64; 10] = &[
    1.0,  2.0,  3.0,  4.0,  5.0,
    6.0,  7.0,  8.0,  9.0,  10.0,
];

const SQUARE: &'static [f64; 10] = &[
    1.0,  3.0,  5.0,  7.0,  9.0,
    11.0, 13.0, 15.0, 17.0, 19.0,
];

const TAU: f64 = PI * 2.0;

#[derive(Copy, Clone, Debug)]
pub struct Wave {
    amplitude: f64, 
    frequency: f64,
    time: f64,
    shape: Shape,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Shape {
    Silence,
    Sine,
    Sawtooth,
    Square,
}

impl Shape {
    fn coefficients(&self) -> impl Iterator<Item = &'static f64> {
        match self {
        | Shape::Silence => SILENCE.iter(),
        | Shape::Sine => SINE.iter(),
        | Shape::Sawtooth => SAWTOOTH.iter(),
        | Shape::Square => SQUARE.iter(),
        }
    }

    fn size(&self) -> f64 {
        match self {
        | Shape::Silence => SILENCE.len() as f64,
        | Shape::Sine => SINE.len() as f64,
        | Shape::Sawtooth => SAWTOOTH.len() as f64,
        | Shape::Square => SQUARE.len() as f64,
        }
    }
}

impl Iterator for Wave {

    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.shape.coefficients()
            .map(|coefficient| self.amplitude * (coefficient * TAU * self.frequency * self.time).sin())
            .sum::<f64>() / self.shape.size();

        self.time += 1.0 / SAMPLE_RATE;
        Some(next)
    }
}
