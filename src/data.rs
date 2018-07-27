use num_traits::NumOps;

pub struct Waveform<I: NumOps, T: Iterator<Item = I>>(T);

impl <I: NumOps, T: Iterator<Item = I>> Waveform<I, T> {
    pub fn from(iter: T) -> Self {
        Waveform(iter)
    }

    pub fn transform<F, J: NumOps, O: Iterator<Item = J>>(self, f: F) -> Waveform<J, O> where F: Fn(T) -> O {
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
}

impl <I: NumOps, T: Iterator<Item = I>> IntoIterator for Waveform<I, T> {
    type Item = I;
    type IntoIter = T;
    fn into_iter(self) -> Self::IntoIter { self.0 }
}
