use note::*;
use instrument::Instrument;
use rhythm::Beat;
use dynamic::Dynamic;
use sound::Sound;

pub struct Track {
    samples: Vec<(Note, Dynamic, Beat)>,
    dynamic: Dynamic,
    bpm: f64,
}

#[macro_export]
macro_rules! track {
    ( $track_dynamic:expr, $bpm:expr; $( $note:expr => $dynamic:expr, $beat:expr );* $(;)* ) => {
        {
            let mut track = Track::new($track_dynamic, $bpm);
            $(
                track.push($note, $dynamic, $beat);
            )*
            track
        }
    }
}

impl Track {
    pub fn new(dynamic: Dynamic, bpm: f64) -> Self {
        Track {
            samples: Vec::new(),
            dynamic,
            bpm,
        }
    }

    pub fn push(&mut self, note: Note, dynamic: Dynamic, beat: Beat) {
        self.samples.push((note, dynamic, beat));
    }

    pub fn map<F>(&self, f: F) -> Self where F: FnMut((Note, Dynamic, Beat)) -> (Note, Dynamic, Beat) {
        Track {
            samples: self.samples.iter()
                .cloned()
                .map(f)
                .collect(),
            dynamic: self.dynamic,
            bpm: self.bpm,
        }
    }

    pub fn convert<I: Instrument>(&self, instrument: &mut I) -> Sound {
        Sound::chain(
            self.samples.iter()
                .map(|(note, dynamic, beat)| {
                    let dynamic = *dynamic * self.dynamic;
                    let duration = *beat * (60.0 / self.bpm);
                    instrument.sing(*note, dynamic, duration)
                })
        )
    }
}
