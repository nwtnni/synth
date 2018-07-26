pub fn quantize(samples: impl IntoIterator<Item = f64>) -> impl Iterator<Item = i16> {
    const MAX: f64 = i16::max_value() as f64;
    samples.into_iter().map(|sample| (sample * MAX) as i16)
}

