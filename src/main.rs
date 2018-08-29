#[macro_use]
extern crate synth;
extern crate hound;

use synth::filter::{normalize, quantize};
use synth::track::*;
use synth::instrument::Bell;
use synth::rhythm;
use synth::dynamic;
use synth::note::Note;
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
        dynamic::MF, 120.0, 100.0;
        Note(4) => dynamic::MF, rhythm::Q;
        Note(2) => dynamic::MF, rhythm::Q;
        Note(0) => dynamic::MF, rhythm::Q;
        Note(2) => dynamic::MF, rhythm::Q;

        Note(4) => dynamic::MF, rhythm::Q;
        Note(4) => dynamic::MF, rhythm::Q;
        Note(4) => dynamic::MF, rhythm::H;

        Note(2) => dynamic::MF, rhythm::Q;
        Note(2) => dynamic::MF, rhythm::Q;
        Note(2) => dynamic::MF, rhythm::H;

        Note(4) => dynamic::MF, rhythm::Q;
        Note(7) => dynamic::MF, rhythm::Q;
        Note(7) => dynamic::MF, rhythm::H;

        Note(4) => dynamic::MF, rhythm::Q;
        Note(2) => dynamic::MF, rhythm::Q;
        Note(0) => dynamic::MF, rhythm::Q;
        Note(2) => dynamic::MF, rhythm::Q;

        Note(4) => dynamic::MF, rhythm::Q;
        Note(4) => dynamic::MF, rhythm::Q;
        Note(4) => dynamic::MF, rhythm::Q;
        Note(0) => dynamic::MF, rhythm::Q;

        Note(2) => dynamic::MF, rhythm::Q;
        Note(2) => dynamic::MF, rhythm::Q;
        Note(4) => dynamic::MF, rhythm::Q;
        Note(2) => dynamic::MF, rhythm::Q;

        Note(0) => dynamic::MF, rhythm::W;
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
