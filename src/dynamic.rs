use std::ops::Mul;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Dynamic(f64);

pub const PPP: Dynamic = Dynamic(1.00);
pub const PP:  Dynamic = Dynamic(2.50);
pub const P:   Dynamic = Dynamic(5.00);
pub const MP:  Dynamic = Dynamic(7.50);
pub const MF:  Dynamic = Dynamic(10.0);
pub const F:   Dynamic = Dynamic(15.0);
pub const FF:  Dynamic = Dynamic(20.0);
pub const FFF: Dynamic = Dynamic(30.0);

impl Default for Dynamic {
    fn default() -> Self { MF }
}

impl Into<f64> for Dynamic {
    fn into(self) -> f64 {
        self.0
    }
}

impl Mul for Dynamic {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Dynamic(self.0 * rhs.0)
    }
}
