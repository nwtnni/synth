use note::*;
use rhythm::*;

pub type Track = Vec<(Note, Beat)>;

#[macro_export]
macro_rules! track {
    ( $( $note:expr => $beat:expr ),* ) => {
        let mut track = Vec::new();
        $(
            track.push(($note, $beat));
        )*
        track
    }
}
