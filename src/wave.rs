use std::f64::consts::PI;

use config::SAMPLE_RATE;

pub fn add<A, B>(lhs: A, rhs: B) -> impl Iterator<Item = f64>
    where A: IntoIterator<Item = f64>,
          B: IntoIterator<Item = f64>, {

    lhs.into_iter()
        .zip(rhs.into_iter())
        .map(|(a, b)| a + b)
}

pub fn sub<A, B>(lhs: A, rhs: B) -> impl Iterator<Item = f64>
    where A: IntoIterator<Item = f64>,
          B: IntoIterator<Item = f64>, {

    lhs.into_iter()
        .zip(rhs.into_iter())
        .map(|(a, b)| a - b)
}

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
