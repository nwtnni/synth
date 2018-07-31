use config::SAMPLE_RATE;
use wave::Wave;

#[derive(Debug, Clone)]
pub enum Sound {
    Wave(Wave),
    Seq(Vec<Sound>),
    Sum(Vec<Sound>),
    Sub(Box<Sound>, Box<Sound>),
    Clip(f64, Box<Sound>),
}

impl Iterator for Sound {

    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
        | Sound::Wave(wave) => wave.next(),
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
