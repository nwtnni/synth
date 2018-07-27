use num_traits::NumOps;
use num_traits::identities::Zero;

pub struct Waveform<I: NumOps + Zero + Copy, T: Iterator<Item = I>>(T);

impl <I: NumOps + Zero + Copy, T: Iterator<Item = I>> Waveform<I, T> {
    pub fn from(iter: T) -> Self {
        Waveform(iter)
    }

    pub fn transform<F, J: NumOps + Zero + Copy, O: Iterator<Item = J>>(self, f: F) -> Waveform<J, O> where F: Fn(T) -> O {
        Waveform(f(self.0))
    }

    pub fn add<Y: Iterator<Item = I>>(self, waveform: Waveform<I, Y>) -> Waveform<I, impl Iterator<Item = I>> {
        Waveform(self.0.zip(waveform.0).map(|(a, b)| a + b))
    }

    pub fn sub<Y: Iterator<Item = I>>(self, waveform: Waveform<I, Y>) -> Waveform<I, impl Iterator<Item = I>> {
        Waveform(self.0.zip(waveform.0).map(|(a, b)| a - b))
    }

    pub fn take(self, count: usize) -> Waveform<I, impl Iterator<Item = I>> {
        Waveform(self.0.take(count))
    }

    pub fn convolve<Y: Iterator<Item = I>>(self, waveform: Waveform<I, Y>) -> Waveform<I, impl Iterator<Item = I>> {
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

impl <I: NumOps + Zero + Copy, T: Iterator<Item = I>> IntoIterator for Waveform<I, T> {
    type Item = I;
    type IntoIter = T;
    fn into_iter(self) -> Self::IntoIter { self.0 }
}
