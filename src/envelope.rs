use std::f32::consts::PI;

use constants::{DELTA, SAMPLE_RATE};

const TAU: f32 = PI * 2.0;

#[derive(Debug, Copy, Clone)]
pub struct Envelope {
    shape: Shape,
    time: f32,
}

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Line {
        slope: f32,
    },

    Exp {
        ratio: f32,
    },

    Sine {
        amplitude: f32,
        frequency: f32,
    },
}

impl Envelope {
    pub fn linear(slope: f32) -> Self {
        Envelope {
            shape: Shape::Line { slope: slope / SAMPLE_RATE },
            time: 0.0,
        }
    }

    pub fn exponential(ratio: f32) -> Self {
        Envelope {
            shape: Shape::Exp {
                ratio: (ratio.ln() / SAMPLE_RATE).exp(),
            },
            time: 0.0,
        }
    }

    pub fn sine(amplitude: f32, frequency: f32) -> Self {
        Envelope {
            shape: Shape::Sine { amplitude, frequency },
            time: 0.0,
        }
    }
}

impl Iterator for Envelope {

    type Item = Box<Fn(f32) -> f32>;

    fn next(&mut self) -> Option<Self::Item> {
        let time = self.time;
        self.time += DELTA;
        Some(
            match self.shape {
                | Shape::Line{slope} => {
                    Box::new(move |value| value + slope)
                }
                | Shape::Exp{ratio} => {
                    Box::new(move |value| value * ratio)
                }
                | Shape::Sine{amplitude, frequency} => {
                    Box::new(move |value| {
                        let previous = (TAU * frequency * (time - DELTA)).sin();
                        let current = (TAU * frequency * time).sin();
                        value + amplitude * (current - previous)
                    })
                }
            }
        )
    }
}
