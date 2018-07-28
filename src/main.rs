#![recursion_limit="128"]

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

    let waveform = Note::E.with(1.5, 0.45).add(Note::G.with(1.0, 0.45))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::D.with(1.5, 0.45).add(Note::F.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::C.with(1.5, 0.45).add(Note::E.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::D.with(1.5, 0.45).add(Note::F.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::E.with(1.5, 0.45).add(Note::G.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::E.with(1.5, 0.45).add(Note::G.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::E.with(1.5, 0.95).add(Note::G.with(1.0, 0.95)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::D.with(1.5, 0.45).add(Note::F.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::D.with(1.5, 0.45).add(Note::F.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::D.with(1.5, 0.95).add(Note::F.with(1.0, 0.95)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::E.with(1.5, 0.45).add(Note::G.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::G.with(1.5, 0.45).add(Note::B.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::G.with(1.5, 0.95).add(Note::B.with(1.0, 0.95)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::E.with(1.5, 0.45).add(Note::G.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::D.with(1.5, 0.45).add(Note::F.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::C.with(1.5, 0.45).add(Note::E.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::D.with(1.5, 0.45).add(Note::F.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::E.with(1.5, 0.45).add(Note::G.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::E.with(1.5, 0.45).add(Note::G.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::E.with(1.5, 0.45).add(Note::G.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::C.with(1.5, 0.45).add(Note::E.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::D.with(1.5, 0.45).add(Note::F.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::D.with(1.5, 0.45).add(Note::F.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::E.with(1.5, 0.45).add(Note::G.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::D.with(1.5, 0.45).add(Note::F.with(1.0, 0.45)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .append(Note::C.with(1.5, 1.95).add(Note::E.with(1.0, 1.95)))
        .append(Waveform::from(Silence {}).with_duration(0.05))

        .transform(normalize)
        .transform(quantize);

    for sample in waveform {
        writer.write_sample(sample).unwrap();
    }
}
