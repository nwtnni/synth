use note::*;
use instrument::Instrument;
use rhythm::Beat;
use dynamic::Dynamic;
use sound::Sound;

pub type Track = Vec<(Note, Dynamic, Beat)>;

#[macro_export]
macro_rules! track {
    ( $( $note:expr => $dynamic:expr, $beat:expr );* $(;)* ) => {
        {
            let mut track: Track = Vec::new();
            $(
                track.push(($note, $dynamic, $beat));
            )*
            track
        }
    }
}

pub fn convert<I>(track: Track, track_dynamic: Dynamic, bpm: f64, instrument: &mut I) -> Sound
    where I: Instrument {

    let spb = 60.0 / bpm;

    Sound::chain(
        track.into_iter()
            .map(|(note, dynamic, beat)| {
                let dynamic = dynamic * track_dynamic;
                let duration = beat * spb;
                instrument.sing(note, dynamic, duration)
            })
    )
}
