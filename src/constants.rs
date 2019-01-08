pub const SAMPLE_RATE: f32 = 44100.0;
pub const NYQUIST_RATE: f32 = SAMPLE_RATE / 2.0;

pub const DELTA: f32 = 1.0 / SAMPLE_RATE;

pub const TAU: f32 = std::f32::consts::PI * 2.0;
pub const FRAC_2_PI: f32 = std::f32::consts::FRAC_2_PI;
pub const FRAC_4_PI: f32 = std::f32::consts::FRAC_2_PI * 2.0;

pub const C4: f32 = 261.626;
pub const D4: f32 = 293.665;
pub const E4: f32 = 329.628;
pub const F4: f32 = 349.228; 
pub const G4: f32 = 391.995;
pub const A4: f32 = 440.000;
pub const B4: f32 = 493.883;

pub const SEMITONE: f32 = 1.059463094359295264561825294946341700779204317494185628559; 
