use constants::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Note(f64);

pub const C: Note = Note(C4);
pub const D: Note = Note(D4);
pub const E: Note = Note(E4);
pub const F: Note = Note(F4);
pub const G: Note = Note(G4);
pub const A: Note = Note(A4);
pub const B: Note = Note(B4);

impl Note {
    pub fn sharp(self) -> Self {
        Note(self.0 * SEMITONE)
    }

    pub fn flat(self) -> Self {
        Note(self.0 / SEMITONE)
    }

    pub fn up(self, steps: i32) -> Self {
        Note(self.0 * SEMITONE.powi(steps))
    }

    pub fn down(self, steps: i32) -> Self {
        Note(self.0 / SEMITONE.powi(steps))
    }

    pub fn octave_up(self) -> Self {
        Note(self.0 * 2.0)
    }

    pub fn octave_down(self) -> Self {
        Note(self.0 / 2.0)
    }
}

impl Into<f64> for Note {
    fn into(self) -> f64 {
        self.0
    }
}
