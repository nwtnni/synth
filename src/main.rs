#[macro_use]
extern crate pipeline;
extern crate hound;
extern crate synth;

use synth::wave::{add, Sine};
use synth::filter::{normalize, quantize};

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let waveform = pipe! {
        add(
            Sine::new(2.0, 110.0),
            Sine::new(1.0, 440.0),
        ).take(44100)
        => normalize
        => quantize
    };

    for sample in waveform {
        writer.write_sample(sample).unwrap();
    }
}
