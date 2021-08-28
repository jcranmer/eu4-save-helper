use std::fmt;
use std::iter::Sum;

/// A fixed point integer, with a base of 1000.
/// This means that 0.001 + 0.001 = 0.002--there are three decimal places of
/// accuracy.
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Default)]
pub struct FixedPoint(pub(crate) i32);

impl FixedPoint {
    pub const ZERO: Self = FixedPoint(0);
    pub const ONE: Self = FixedPoint(1000);
}

impl fmt::Display for FixedPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        let val = self.0.abs();
        write!(f, "{}{}.{:03}", sign, val / 1000, val % 1000)
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

impl ops::Neg for FixedPoint {
    type Output = FixedPoint;
    fn neg(self) -> FixedPoint {
        Self(-self.0)
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
        self.0 = ((self.0 as i64 * 1000) / other.0 as i64) as i32;
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

impl From<FixedPoint> for f64 {
    fn from(val: FixedPoint) -> Self {
        f64::from(val.0) / 1000.0
    }
}

impl Sum for FixedPoint {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.reduce(ops::Add::add)
            .unwrap_or_default()
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

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let too_few = Self::Err::Format;
        let negation = if &s[0..1] == "-" {
            s = &s[1..];
            -1
        } else {
            1
        };
        let mut pieces = s.split(".");
        let integer : i32 = pieces.next().ok_or(too_few)?.parse()?;
        let fract_str = pieces.next().unwrap_or("0");
        let fract = match fract_str.len() {
            1 => fract_str.parse::<i32>()? * 100,
            2 => fract_str.parse::<i32>()? * 10,
            3 => fract_str.parse::<i32>()?,
            _ => fract_str[0..3].parse::<i32>()?
        };
        if fract > 1000 || fract < 0 {
            return Err(Self::Err::Format);
        }
        Ok(Self(negation * (integer * 1000 + fract)))
    }
}

