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
use synth::scale::*;

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let mut bell = Bell::new(0.005);

    let mut track = Track::new(dynamic::MF, 60.0, 261.626);
    
    for note in Scale::new(Shape::Minor).take(15) {
        track.push(note, dynamic::MF, rhythm::E); 
    }

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
