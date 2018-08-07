use track::Track;
use sound::Sound;

pub trait Instrument {
    fn sing(track: Track) -> Sound;
}

pub struct Bell {

}
