use config::SAMPLE_RATE;
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
    fn apply_to_waves(&mut self, mode: Mode, f: &Fn(f64, f64) -> f64) {
        match self {
        | Sound::Wave(wave) => wave.apply(mode, f),
        | Sound::Env(_, _, sound) => sound.apply_to_waves(mode, f),
        | Sound::Seq(sounds) => for sound in sounds { sound.apply_to_waves(mode, f) },
        | Sound::Sum(sounds) => for sound in sounds { sound.apply_to_waves(mode, f) },
        | Sound::Sub(l, r) => { l.apply_to_waves(mode, f); r.apply_to_waves(mode, f); },
        | Sound::Clip(_, sound) => sound.apply_to_waves(mode, f),
        }
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
                sound.apply_to_waves(*mode, &*f);
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
