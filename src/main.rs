#[macro_use]
extern crate synth;
extern crate hound;

use synth::filter::{normalize, quantize};
use synth::track::*;
use synth::instrument::Bell;
use synth::rhythm;
use synth::dynamic;
use synth::note;

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let mut bell = Bell::new(0.005);

    let track = track! {
        note::E => dynamic::MF, rhythm::Q;
        note::D => dynamic::MF, rhythm::Q;
        note::C => dynamic::MF, rhythm::Q;
        note::D => dynamic::MF, rhythm::Q;

        note::E => dynamic::MF, rhythm::Q;
        note::E => dynamic::MF, rhythm::Q;
        note::E => dynamic::MF, rhythm::H;

        note::D => dynamic::MF, rhythm::Q;
        note::D => dynamic::MF, rhythm::Q;
        note::D => dynamic::MF, rhythm::H;

        note::E => dynamic::MF, rhythm::Q;
        note::G => dynamic::MF, rhythm::Q;
        note::G => dynamic::MF, rhythm::H;

        note::E => dynamic::MF, rhythm::Q;
        note::D => dynamic::MF, rhythm::Q;
        note::C => dynamic::MF, rhythm::Q;
        note::D => dynamic::MF, rhythm::Q;

        note::E => dynamic::MF, rhythm::Q;
        note::E => dynamic::MF, rhythm::Q;
        note::E => dynamic::MF, rhythm::Q;
        note::C => dynamic::MF, rhythm::Q;

        note::D => dynamic::MF, rhythm::Q;
        note::D => dynamic::MF, rhythm::Q;
        note::E => dynamic::MF, rhythm::Q;
        note::D => dynamic::MF, rhythm::Q;

        note::C => dynamic::MF, rhythm::W;
    };

    let waveform = convert(track, dynamic::MF, 120.0, &mut bell);

    for sample in quantize(normalize(waveform)) {
        writer.write_sample(sample).unwrap();
    }
}
