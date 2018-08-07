# synth

Sound is a great playground for tinkering with functional ideas:
composition and abstraction can be found everywhere in music. This is
an attempt to rediscover and rebuild these abstractions, learning
about music from the bottom up.

## Wave

```rust
pub struct Wave {
    shape: Shape,
    amplitude: f64, 
    frequency: f64,
    time: f64,
}

pub enum Shape {
    Silence,
    Sine,
    Sawtooth,
    Square,
}
```

Waves--specifically sine waves--are the primitive building block of sound.
These are just iterators that produce a single amplitude per iteration.

## Envelope

```rust
pub struct Envelope {
    shape: Shape,
    time: f64,
}

pub enum Shape {
    Line { slope: f64 },
    Exp  { ratio: f64 },
    Sine { amplitude: f64, frequency: f64 },
}
```

Envelopes modulate waveforms, making them oscillate or grow, for example.

## Sound

```rust
pub enum Sound {
    Wave(Wave),
    Env(Mode, Envelope, Box<Sound>),
    Seq(Vec<Sound>),
    Sum(Vec<Sound>),
    Sub(Box<Sound>, Box<Sound>),
    Clip(f64, Box<Sound>),
}
```

Inspired by compilers and programming languages, this is effectively an
intermediate representation for sound. Tree-structured data can be
composed, nested, and traversed: we can easily overlay two
trees, chain them end-to-end, or wrap them in envelopes.

## Note, Dynamic, Rhythm

These are just wrappers for frequency, amplitude, and duration, respectively.

Lifting them into wrapper types allows us to express more abstract ideas; for example:

```rust
let a_sharp     = note::A.sharp().octave_up();
let forte       = dynamic::F;
let dotted_half = rhythm::H.dotted();
```

## Instrument

Instruments (theoretically) convert a given note, dynamic, and rhythm into actual sound.

```rust
pub trait Instrument {
    fn sing(&mut self, note: Note, dynamic: Dynamic, sec: f64) -> Sound;
}
```

## Track

Tracks are a sequence of notes with their own BPM and relative dynamic.
