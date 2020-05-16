use std::fmt;

/// A fixed point integer, with a base of 1000.
/// This means that 0.001 + 0.001 = 0.002--there are three decimal places of
/// accuracy.
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

#[derive(thiserror::Error, Debug)]
pub enum ParseFixedPointError {
    #[error("bad format")]
    Format,
    #[error("bad format")]
    Int(#[from] std::num::ParseIntError)
}

impl std::str::FromStr for FixedPoint {
    type Err = ParseFixedPointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let too_few = Self::Err::Format;
        let mut pieces = s.split(".");
        let integer : i32 = pieces.next().ok_or(too_few)?.parse()?;
        let fract : i32 = pieces.next().unwrap_or("0").parse()?;
        if fract > 1000 || fract < 0 {
            return Err(Self::Err::Format);
        }
        Ok(Self(integer * 1000 + fract))
    }
}

