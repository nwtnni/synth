#[macro_use]
extern crate synth;
extern crate hound;

use synth::filter::{normalize, quantize};
use synth::track::*;
use synth::instrument::Bell;
use synth::rhythm;
use synth::dynamic;
use synth::note;
use synth::sound;

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
        dynamic::MF, 120.0;
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

    let waveform = sound::Sound::Sum(vec![
        track.convert(&mut bell),
        track.map(|note, dynamic, beat| {
            (note.octave_up(), dynamic, beat)
        })
        .convert(&mut bell),
    ]);

    for sample in quantize(normalize(waveform)) {
        writer.write_sample(sample).unwrap();
    }
}
