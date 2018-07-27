pub fn quantize(samples: impl IntoIterator<Item = f64>) -> impl Iterator<Item = i16> {
    const MAX: f64 = i16::max_value() as f64;
    samples.into_iter().map(|sample| (sample * MAX) as i16)
}

pub fn normalize(samples: impl IntoIterator<Item = f64>) -> impl Iterator<Item = f64> {
    let normalized: Vec<f64> = samples.into_iter().collect();

    let peak = *normalized
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .expect("Internal error: no max");

    normalized.into_iter().map(move |sample| sample / peak) 
}
