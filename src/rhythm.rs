pub struct Beat(f64);

pub const W: Beat = Beat(4.0000);
pub const H: Beat = Beat(2.0000);
pub const Q: Beat = Beat(1.0000);
pub const E: Beat = Beat(0.1250);
pub const S: Beat = Beat(0.0625);
pub const T: Beat = Beat(0.3333333333333333333333333333);

impl Beat {
    pub fn dotted(self) -> Self {
        Beat(self.0 * 1.5)
    }

    pub fn double(self) -> Self {
        Beat(self.0 * 2.0)
    }

    pub fn halve(self) -> Self {
        Beat(self.0 / 2.0)
    }
}
