use note::Note;
use sound::Sound;
use wave::{Mode, Shape, Wave};
use envelope::Envelope;
use dynamic::Dynamic;

pub trait Instrument {
    fn sing(&mut self, note: Note, dynamic: Dynamic, sec: f32, key: f32) -> Sound;
}

pub struct Bell {
    decay: f32,
}

impl Bell {
    pub fn new(decay: f32) -> Self {
        Bell { decay }
    }
}

impl Instrument for Bell {
    fn sing(&mut self, note: Note, dynamic: Dynamic, sec: f32, key: f32) -> Sound {
        Sound::Wave(
            Wave::new(
                Shape::Sawtooth,
                dynamic.into(),
                note.in_key(key),
            )
        )
        .envelop(
            Mode::Amplitude,
            Envelope::exponential(self.decay),
        )
        .clip(sec)
    }
}
