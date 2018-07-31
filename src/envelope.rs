use std::f64::consts::PI;

use config::SAMPLE_RATE;

const TAU: f64 = PI * 2.0;

#[derive(Debug, Copy, Clone)]
pub struct Envelope {
    shape: Shape,
    time: f64,
    start: f64,
    stop: f64,
}

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Line {
        offset: f64, 
        slope: f64,
    },

    Exp {
        ratio: f64,
    },

    Sine {
        amplitude: f64,
        frequency: f64,
        offset: f64,
    },
}

impl Envelope {
    pub fn linear(offset: f64, slope: f64, start: f64, stop: f64) -> Self {
        Envelope {
            shape: Shape::Line { offset, slope },
            time: 0.0,
            start,
            stop,
        }
    }

    pub fn exponential(ratio: f64, start: f64, stop: f64) -> Self {
        Envelope {
            shape: Shape::Exp { ratio },
            time: 0.0,
            start,
            stop,
        }
    }

    pub fn sine(amplitude: f64, frequency: f64, offset: f64, start: f64, stop: f64) -> Self {
        Envelope {
            shape: Shape::Sine { amplitude, frequency, offset },
            time: 0.0,
            start,
            stop,
        }
    }
}

impl Iterator for Envelope {

    type Item = Box<Fn(f64, f64) -> f64>;

    fn next(&mut self) -> Option<Self::Item> {
        let time = self.time;

        self.time += 1.0 / SAMPLE_RATE;

        if time < self.start || time > self.stop { return None }

        Some(
            match self.shape {
                | Shape::Line{offset, slope} => {
                    Box::new(move |initial, _| offset + initial + (slope * time))
                }
                | Shape::Exp{ratio} => {
                    Box::new(move |_, current| current * ratio)
                }
                | Shape::Sine{amplitude, frequency, offset} => {
                    Box::new(move |initial, _| {
                        initial + offset + (amplitude * (TAU * frequency * time).sin())
                    })
                }
            }
        )
    }
}
