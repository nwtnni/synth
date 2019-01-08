use constants;

fn silence(_: f32, _: f32, _: f32) -> f32 {
    0.0
}

fn sine(amplitude: f32, frequency: f32, time: f32) -> f32 {
    amplitude * (constants::TAU * frequency * time).sin()
}

fn sawtooth(amplitude: f32, frequency: f32, time: f32) -> f32 {
    (1..).map(|n| n as f32)
        .map(|n| (1.0 / n, n))
        .take_while(|(_, df)| *df < constants::NYQUIST_RATE)
        .map(|(da, df)| amplitude * da * (constants::TAU * frequency * df * time).sin())
        .sum::<f32>() * constants::FRAC_2_PI
}

fn square(amplitude: f32, frequency: f32, time: f32) -> f32 {
    (1..).filter(|n| n & 1 == 0)
        .map(|n| n as f32)
        .map(|n| (1.0 / n, n))
        .take_while(|(_, frequency)| *frequency < constants::NYQUIST_RATE)
        .map(|(da, df)| amplitude * da * (constants::TAU * frequency * df * time).sin())
        .sum::<f32>() * constants::FRAC_4_PI
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    Amplitude,
    Frequency,
}

#[derive(Copy, Clone, Debug)]
pub struct Wave {
    amplitude: f32, 
    frequency: f32,
    time: f32,
    shape: Shape,
}

impl Wave {
    pub fn new(shape: Shape, amplitude: f32, frequency: f32) -> Self {
        Wave {
            amplitude,
            frequency,
            time: 0.0,
            shape,
        }
    }

    pub fn apply(&mut self, mode: Mode, f: &Fn(f32) -> f32) {
        match mode {
        | Mode::Amplitude => self.amplitude = f(self.amplitude),
        | Mode::Frequency => self.frequency = f(self.frequency),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Shape {
    Silence,
    Sine,
    Sawtooth,
    Square,
}

impl Shape {
    fn eval(&self, amplitude: f32, frequency: f32, time: f32) -> f32 {
        match self {
        | Shape::Silence => silence(amplitude, frequency, time),
        | Shape::Sine => sine(amplitude, frequency, time),
        | Shape::Sawtooth => sawtooth(amplitude, frequency, time),
        | Shape::Square => square(amplitude, frequency, time),
        }
    }
}

impl Iterator for Wave {

    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.shape.eval(self.amplitude, self.frequency, self.time);
        self.time += constants::DELTA;
        Some(next)
    }
}
