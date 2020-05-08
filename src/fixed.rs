use std::fmt;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Default)]
pub struct FixedPoint(i32);

impl fmt::Display for FixedPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:03}", self.0 / 1000, self.0 % 1000)
    }
}

impl fmt::Debug for FixedPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

use std::ops;
impl ops::Add for FixedPoint {
    type Output = FixedPoint;
    fn add(self, other: FixedPoint) -> FixedPoint {
        let mut res = self;
        res += other;
        res
    }
}

impl ops::Sub for FixedPoint {
    type Output = FixedPoint;
    fn sub(self, other: FixedPoint) -> FixedPoint {
        let mut res = self;
        res -= other;
        res
    }
}

impl ops::Mul for FixedPoint {
    type Output = FixedPoint;
    fn mul(self, other: FixedPoint) -> FixedPoint {
        let mut res = self;
        res *= other;
        res
    }
}

impl ops::Div for FixedPoint {
    type Output = FixedPoint;
    fn div(self, other: FixedPoint) -> FixedPoint {
        let mut res = self;
        res /= other;
        res
    }
}

impl ops::AddAssign for FixedPoint {
    fn add_assign(&mut self, other: FixedPoint) {
        self.0 += other.0;
    }
}

impl ops::SubAssign for FixedPoint {
    fn sub_assign(&mut self, other: FixedPoint) {
        self.0 -= other.0;
    }
}

impl ops::MulAssign for FixedPoint {
    fn mul_assign(&mut self, other: FixedPoint) {
        self.0 = ((self.0 as i64 * other.0 as i64) / 1000) as i32;
    }
}

impl ops::DivAssign for FixedPoint {
    fn div_assign(&mut self, other: FixedPoint) {
        self.0 = ((other.0 as i64 * 1000) / self.0 as i64) as i32;
    }
}

impl From<i32> for FixedPoint {
    fn from(val: i32) -> Self {
        FixedPoint(val * 1000)
    }
}

impl From<f32> for FixedPoint {
    fn from(val: f32) -> Self {
        FixedPoint((val * 1000.0) as i32)
    }
}
