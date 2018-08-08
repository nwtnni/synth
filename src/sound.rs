use constants::SAMPLE_RATE;
use envelope::Envelope;
use wave::{Mode, Wave};

#[derive(Debug, Clone)]
pub enum Sound {
    Wave(Wave),
    Env(Mode, Envelope, Box<Sound>),
    Seq(Vec<Sound>),
    Sum(Vec<Sound>),
    Sub(Box<Sound>, Box<Sound>),
    Clip(f64, Box<Sound>),
}

impl Sound {
    pub fn clip(self, time: f64) -> Self {
        Sound::Clip(time, Box::new(self))
    }

    pub fn envelop(self, mode: Mode, envelope: Envelope) -> Self {
        Sound::Env(mode, envelope, Box::new(self)) 
    }

    pub fn sum<I>(sounds: I) -> Self where I: IntoIterator<Item = Self> {
        Sound::Sum(sounds.into_iter().collect())
    }

    pub fn chain<I>(sounds: I) -> Self where I: IntoIterator<Item = Self> {
        Sound::Seq(sounds.into_iter().collect())
    }

    pub fn repeat(&self, times: usize) -> Self {
        let mut seq = Vec::new();
        for _ in 0..times {
            seq.push(self.clone());
        }
        Sound::Seq(seq)
    }

    fn apply(&mut self, mode: Mode, f: &Fn(f64) -> f64) {
        match self {
        | Sound::Wave(wave) => wave.apply(mode, f),
        | Sound::Env(_, _, sound) => sound.apply(mode, f),
        | Sound::Seq(sounds) => for sound in sounds { sound.apply(mode, f) },
        | Sound::Sum(sounds) => for sound in sounds { sound.apply(mode, f) },
        | Sound::Sub(l, r) => { l.apply(mode, f); r.apply(mode, f); },
        | Sound::Clip(_, sound) => sound.apply(mode, f),
        }
    }
}

impl Into<Sound> for Wave {
    fn into(self) -> Sound {
        Sound::Wave(self)
    }
}

impl Iterator for Sound {

    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
        | Sound::Wave(wave) => wave.next(),
        | Sound::Env(mode, env, sound) => {

            let next = sound.next();
            if let Some(f) = env.next() {
                sound.apply(*mode, &*f);
                next
            } else {
                None
            }

        },
        | Sound::Seq(sounds) => {
            loop {
                if let Some(sound) = sounds.first_mut() {
                    if let Some(next) = sound.next() {
                        return Some(next)
                    }
                } else {
                    return None
                }
                sounds.remove(0);
            }
        }
        | Sound::Sum(sounds) => {
            sounds.iter_mut()
                .map(|sound| sound.next())
                .fold(Some(0.0), |a, b| {
                    match (a, b) {
                    | (Some(a), Some(b)) => Some(a + b),
                    | _ => None,
                    }
                })
        }
        | Sound::Sub(left, right) => {
            match (left.next(), right.next()) {
            | (Some(a), Some(b)) => Some(a - b),
            | _ => None,
            }
        }
        | Sound::Clip(duration, sound) => {
            if *duration <= 0.0 { return None }
            *duration -= 1.0 / SAMPLE_RATE;
            sound.next()
        }
        }
    }
}
