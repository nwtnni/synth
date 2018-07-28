use num_traits::NumOps;
use num_traits::identities::Zero;

use config::SAMPLE_RATE;

#[derive(Copy, Clone)]
pub struct Waveform<I: NumOps + Zero + Copy + Clone, T: Iterator<Item = I> + Clone>(T);

impl <I: NumOps + Zero + Copy, T: Iterator<Item = I> + Clone> Waveform<I, T> {
    pub fn from(iter: T) -> Self {
        Waveform(iter)
    }

    pub fn transform<F, J: NumOps + Zero + Copy, O: Iterator<Item = J> + Clone>(self, f: F) -> Waveform<J, O> where F: Fn(T) -> O {
        Waveform(f(self.0))
    }

    pub fn add<Y: Iterator<Item = I> + Clone>(self, waveform: Waveform<I, Y>) -> Waveform<I, impl Iterator<Item = I> + Clone> {
        Waveform(self.0.zip(waveform.0).map(|(a, b)| a + b))
    }

    pub fn sub<Y: Iterator<Item = I> + Clone>(self, waveform: Waveform<I, Y>) -> Waveform<I, impl Iterator<Item = I> + Clone> {
        Waveform(self.0.zip(waveform.0).map(|(a, b)| a - b))
    }

    pub fn append<Y: Iterator<Item = I> + Clone>(self, waveform: Waveform<I, Y>) -> Waveform<I, impl Iterator<Item = I> + Clone> {
        Waveform(self.0.chain(waveform.0))
    }

    pub fn with_duration(self, duration: f64) -> Waveform<I, impl Iterator<Item = I> + Clone> {
        Waveform(self.0.take((duration * SAMPLE_RATE) as usize))
    }

    pub fn cycle(self) -> Waveform<I, impl Iterator<Item = I> + Clone> {
        Waveform(self.0.cycle())
    }

    pub fn convolve<Y: Iterator<Item = I> + Clone>(self, waveform: Waveform<I, Y>) -> Waveform<I, impl Iterator<Item = I> + Clone> {
        let lhs = self.into_iter().collect::<Vec<_>>();
        let rhs = waveform.into_iter().collect::<Vec<_>>();
        let rhs_half = rhs.len() / 2;

        Waveform((0..lhs.len()).map(move |i| {
            rhs.iter()
                .enumerate()
                .map(|(j, r)| *r * *lhs.get(i - j + rhs_half).unwrap_or(&I::zero()))
                .fold(I::zero(), |a, b| a + b)
        }))
    }
}

impl <I: NumOps + Zero + Copy, T: Iterator<Item = I> + Clone> IntoIterator for Waveform<I, T> {
    type Item = I;
    type IntoIter = T;
    fn into_iter(self) -> Self::IntoIter { self.0 }
}
