extern crate hound;
extern crate synth;

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

    for sample in quantize(normalize(Sine::new(10000.0, 440.0).take(44100))) {
        writer.write_sample(sample).unwrap();
    }
}
