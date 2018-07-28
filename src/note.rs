use data::Waveform;
use wave::*;

pub enum Note {
    A, B, C, D, E, F, G,
}

impl Note {
    pub fn with(self, amplitude: f64, duration: f64) -> Waveform<f64, impl Iterator<Item = f64> + Clone> {
        let frequency = match self {
        | Note::A => 440.0,
        | Note::B => 494.0,
        | Note::C => 523.0,
        | Note::D => 587.0,
        | Note::E => 659.0,
        | Note::F => 698.0,
        | Note::G => 784.0,
        };
        
        Waveform::from(Sine::new(amplitude, frequency))
            .with_duration(duration)
    }
}
