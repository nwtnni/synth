use note::*;
use instrument::Instrument;
use rhythm::Beat;
use dynamic::Dynamic;
use sound::Sound;

pub struct Track {
    samples: Vec<(Note, Dynamic, Beat)>,
    dynamic: Dynamic,
    bpm: f32,
    key: f32,
}

#[macro_export]
macro_rules! track {
    ( $track_dynamic:expr, $bpm:expr, $key:expr; $( $note:expr => $dynamic:expr, $beat:expr );* $(;)* ) => {
        {
            let mut track = Track::new($track_dynamic, $bpm, $key);
            $(
                track.push($note, $dynamic, $beat);
            )*
            track
        }
    }
}

impl Track {
    pub fn new(dynamic: Dynamic, bpm: f32, key: f32) -> Self {
        Track {
            samples: Vec::new(),
            dynamic,
            bpm,
            key,
        }
    }

    pub fn push(&mut self, note: Note, dynamic: Dynamic, beat: Beat) {
        self.samples.push((note, dynamic, beat));
    }

    pub fn map<F>(&self, mut f: F) -> Self where F: FnMut(Note, Dynamic, Beat) -> (Note, Dynamic, Beat) {
        Track {
            samples: self.samples.iter()
                .cloned()
                .map(|(n, d, b)| f(n, d, b))
                .collect(),
            dynamic: self.dynamic,
            bpm: self.bpm,
            key: self.key,
        }
    }

    pub fn map_melody<F>(&self, mut f: F) -> Self where F: FnMut(Note) -> Note {
        self.map(|n, d, b| (f(n), d, b))
    }

    pub fn map_dynamic<F>(&self, mut f: F) -> Self where F: FnMut(Dynamic) -> Dynamic {
        self.map(|n, d, b| (n, f(d), b))
    }

    pub fn map_rhythm<F>(&self, mut f: F) -> Self where F: FnMut(Beat) -> Beat {
        self.map(|n, d, b| (n, d, f(b)))
    }

    pub fn convert<I: Instrument>(&self, instrument: &mut I) -> Sound {
        Sound::chain(
            self.samples.iter()
                .map(|(note, dynamic, beat)| {
                    let dynamic = *dynamic * self.dynamic;
                    let duration = *beat * (60.0 / self.bpm);
                    instrument.sing(*note, dynamic, duration, self.key)
                })
        )
    }
}
