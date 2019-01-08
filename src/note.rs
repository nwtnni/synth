use constants::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Note(pub i32);

impl Note {
    pub fn sharp(self) -> Self {
        Note(self.0 + 1)
    }

    pub fn flat(self) -> Self {
        Note(self.0 - 1)
    }

    pub fn up(self, steps: i32) -> Self {
        Note(self.0 + steps)
    }

    pub fn down(self, steps: i32) -> Self {
        Note(self.0 - steps)
    }

    pub fn octave_up(self) -> Self {
        Note(self.0 + 12)
    }

    pub fn octave_down(self) -> Self {
        Note(self.0 - 12)
    }

    pub fn in_key(self, key: f32) -> f32 {
        key * SEMITONE.powi(self.0)
    }
}
