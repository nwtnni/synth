pub fn quantize<I: Iterator<Item = f32> + Clone>(samples: I) -> impl Iterator<Item = i16> + Clone {
    const MAX: f32 = i16::max_value() as f32;
    samples.into_iter().map(|sample| (sample * MAX) as i16)
}

pub fn normalize<I: Iterator<Item = f32> + Clone>(samples: I) -> impl Iterator<Item = f32> + Clone {
    let normalized: Vec<f32> = samples.into_iter().collect();

    let peak = *normalized
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .expect("Internal error: no max") * 1.10;

    normalized.into_iter().map(move |sample| sample / peak) 
}
