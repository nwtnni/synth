pub const SAMPLE_RATE: f64 = 44100.0;
pub const NYQUIST_RATE: f64 = SAMPLE_RATE / 2.0;

pub const DELTA: f64 = 1.0 / SAMPLE_RATE;

pub const TAU: f64 = std::f64::consts::PI * 2.0;
pub const FRAC_2_PI: f64 = std::f64::consts::FRAC_2_PI;
pub const FRAC_4_PI: f64 = std::f64::consts::FRAC_2_PI * 2.0;

pub const C4: f64 = 261.626;
pub const D4: f64 = 293.665;
pub const E4: f64 = 329.628;
pub const F4: f64 = 349.228; 
pub const G4: f64 = 391.995;
pub const A4: f64 = 440.000;
pub const B4: f64 = 493.883;

pub const SEMITONE: f64 = 1.059463094359295264561825294946341700779204317494185628559; 
