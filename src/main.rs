extern crate hound;
extern crate synth;

use synth::data::Waveform;
use synth::wave::*;
use synth::note::*;
use synth::filter::{normalize, quantize};

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let waveform = Note::new(Note::E, 1.0).with_duration(0.45).add(Note::new(Note::G, 1.0).with_duration(0.45))
        .append(Waveform::from(Silence {}).with_duration(0.05))
        .append(
            Note::new(Note::D, 1.0).with_duration(0.45).add(Note::new(Note::F, 1.0).with_duration(0.45))
        )
        .append(Waveform::from(Silence {}).with_duration(0.05))
        .append(
            Note::new(Note::C, 1.0).with_duration(0.45).add(Note::new(Note::E, 1.0).with_duration(0.45))
        )
        .append(Waveform::from(Silence {}).with_duration(0.05))
        .append(
            Note::new(Note::D, 1.0).with_duration(0.45).add(Note::new(Note::F, 1.0).with_duration(0.45))
        )
        .append(Waveform::from(Silence {}).with_duration(0.05))
        // .append(
        //     Note::new(Note::E, 1.0).with_duration(0.45).add(Note::new(Note::G, 1.0).with_duration(0.45))
        // )
        // .append(Waveform::from(Silence {}).with_duration(0.05))
        .cycle()
        .with_duration(8.0)
        .transform(normalize)
        .transform(quantize);

    for sample in waveform {
        writer.write_sample(sample).unwrap();
    }
}
