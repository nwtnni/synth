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
            Wave::new(Shape::Sawtooth, 1.0, 440.0).into(),
            Wave::new(Shape::Sawtooth, 1.0, 554.37).into(),
            Wave::new(Shape::Sawtooth, 1.0, 659.25).into(),
        ])
        // .envelop(
        //     Mode::Amplitude,
        //     Envelope::sine(1.0, 0.5),
        // )
        .envelop(
            Mode::Amplitude,
            Envelope::exponential(0.025),
        )
        // .envelop(
        //     Mode::Frequency,
        //     Envelope::linear(10.0),
        // )
        // .envelop(
        //     Mode::Frequency,
        //     Envelope::sine(0.01, 5.0),
        // )
        .clip(0.5);

    for sample in quantize(normalize(waveform)) {
        writer.write_sample(sample).unwrap();
    }
}
