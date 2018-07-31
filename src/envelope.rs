use config::SAMPLE_RATE;

pub struct Envelope {
    shape: Shape,
    time: f64,
    start: f64,
    stop: f64,
}

pub enum Shape {
    Line {
        offset: f64, 
        slope: f64,
    },

    Exp {
        ratio: f64,
    },
}

impl Envelope {
    pub fn linear(offset: f64, slope: f64, start: f64, stop: f64) -> Self {
        Envelope {
            shape: Shape::Line {
                offset,
                slope
            },
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
            }
        )
    }
}
