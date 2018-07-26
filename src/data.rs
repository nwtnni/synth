use std::ops::{Add, Sub};
use std::iter;
use std::vec;
use std::slice;

pub struct Waveform {
    data: Vec<f64>,
}

impl Waveform {
    pub fn from<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        Waveform { data: iter.into_iter().collect() }
    }

    pub fn transform<F, T>(self, f: F) -> Self
        where T: IntoIterator<Item = f64>,
              F: Fn(Vec<f64>) -> T {
        
        Waveform { data: f(self.data).into_iter().collect() }
    }
}

impl IntoIterator for Waveform {
    type Item = f64;
    type IntoIter = vec::IntoIter<f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl <'a> IntoIterator for &'a Waveform {
    type Item = &'a f64;
    type IntoIter = slice::Iter<'a, f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl Add for Waveform {
    type Output = Waveform;

    fn add(self, rhs: Self) -> Self::Output {
        Waveform {
            data: self.data.into_iter()
                    .zip(rhs.data)
                    .map(|(a, b)| a + b)
                    .collect()
        }
    }
}

impl <'a> Add for &'a Waveform {
    type Output = Waveform;

    fn add(self, rhs: Self) -> Self::Output {
        Waveform {
            data: self.data.iter()
                    .zip(&rhs.data)
                    .map(|(a, b)| a + b)
                    .collect()
        }
    }
}

impl Sub for Waveform {
    type Output = Waveform;

    fn sub(self, rhs: Self) -> Self::Output {
        Waveform {
            data: self.data.into_iter()
                    .zip(rhs.data)
                    .map(|(a, b)| a - b)
                    .collect()
        }
    }
}

impl <'a> Sub for &'a Waveform {
    type Output = Waveform;

    fn sub(self, rhs: Self) -> Self::Output {
        Waveform {
            data: self.data.iter()
                    .zip(&rhs.data)
                    .map(|(a, b)| a - b)
                    .collect()
        }
    }
}

impl iter::Sum for Waveform {
    fn sum<I>(mut iter: I) -> Self where I: Iterator<Item = Self> {
        let waveform = iter.next();
        if let Some(data) = waveform {
            iter.fold(data, |a, b| a + b)
        } else {
            Waveform { data: Vec::new() }
        }
    }
}
