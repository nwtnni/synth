extern crate hound;
extern crate synth;

use synth::data::Waveform;
use synth::wave::Sine;
use synth::filter::{normalize, quantize};

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let waveform = Waveform::from(Sine::new(50.0, 110.0))
        .add(Waveform::from(Sine::new(25.0, 220.0)))
        .add(Waveform::from(Sine::new(12.5, 440.0)))
        .take(44100)
        .transform(normalize)
        .transform(quantize);

    for sample in waveform {
        writer.write_sample(sample).unwrap();
    }
}
