extern crate hound;
extern crate synth;

use synth::data::Waveform;
use synth::wave::*;
use synth::filter::{normalize, quantize};

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let waveform = Waveform::from(Square::new(1.0, 440.0))
        .take(44100 * 4)
        .transform(normalize)
        .transform(quantize);

    for sample in waveform {
        writer.write_sample(sample).unwrap();
    }
}
