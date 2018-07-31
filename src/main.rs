#![recursion_limit="128"]

extern crate hound;
extern crate synth;

use synth::sound::*;
use synth::filter::{normalize, quantize};
use synth::wave::{Mode, Wave, Shape};
use synth::envelope::Envelope;

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let waveform = Sound::sum(vec![
            Wave::new(Shape::Sine, 2.0, 440.0).into(),
            Wave::new(Shape::Sine, 1.0, 554.37).into(),
            Wave::new(Shape::Sine, 0.5, 659.25).into(),
        ])
        .envelop(
            Mode::Frequency,
            Envelope::sine(0.0, 0.20, 5.0, 0.0, 100.0),
        )
        .envelop(
            Mode::Amplitude,
            Envelope::exponential(0.30, 0.0, 100.0),
        )
        .clip(5.0);

    for sample in quantize(normalize(waveform)) {
        writer.write_sample(sample).unwrap();
    }
}
