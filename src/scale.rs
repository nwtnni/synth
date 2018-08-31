use note::Note;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Scale {
    note: Note,
    shape: Shape,
}

impl Scale {
    pub fn new(shape: Shape) -> Self {
        Scale {
            note: Note(0),
            shape,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Shape {
    Major,
    Minor,
}

impl Shape {
    pub fn next(&self, note: Note) -> Note {
        let next = match self {
        | Shape::Major => {
            match note.0 % 12 {
            | 0 => 2,
            | 2 => 2,
            | 4 => 1,
            | 5 => 2,
            | 7 => 2,
            | 9 => 2,
            | 11 => 1,
            | _ => panic!("Internal error: misaligned major scale"),
            }
        },
        | Shape::Minor => {
            match note.0 % 12 {
            | 0 => 2,
            | 2 => 1,
            | 3 => 2,
            | 5 => 2,
            | 7 => 1,
            | 8 => 2,
            | 10 => 2,
            | _ => panic!("Internal error: misaligned minor scale")
            }
        },
        };

        Note(next + note.0)
    }
}

impl Iterator for Scale {
    type Item = Note;
    fn next(&mut self) -> Option<Self::Item> {
        let note = self.note;
        self.note = self.shape.next(self.note);
        Some(note)
    }
}
