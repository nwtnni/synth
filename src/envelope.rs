pub enum Envelope {
    Line {
        offset: f64, 
        slope: f64,
    },

    Exp {
        ratio: f64,
    },
}

impl Envelope {
    pub fn linear(offset: f64, slope: f64) -> Self {
        Envelope::Line { offset, slope }
    }

    pub fn exponential(ratio: f64) -> Self {
        Envelope::Exp { ratio }
    }
}
