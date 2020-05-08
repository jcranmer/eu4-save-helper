use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.year, self.month, self.day)
    }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Default for Date {
    fn default() -> Self {
        Date { year: 1, month: 1, day: 1 }
    }
}

#[derive(Copy, Clone)]
pub struct ParseDateError {
    msg: &'static str
}

impl fmt::Debug for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<std::num::ParseIntError> for ParseDateError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self { msg: "Date component not an integer" }
    }
}

impl std::str::FromStr for Date {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let too_few = Self::Err { msg: "Too few date components" };
        let mut pieces = s.split(".");
        let year = pieces.next().ok_or(too_few)?.parse()?;
        let month = pieces.next().ok_or(too_few)?.parse()?;
        let day = pieces.next().ok_or(too_few)?.parse()?;
        if pieces.next().is_some() {
            return Err(Self::Err { msg: "Too many date components" });
        }
        if year < 1 || year > 9999 {
            return Err(Self::Err { msg: "Year out of range" });
        }
        if month < 1 || month > 12 {
            return Err(Self::Err { msg: "Month out of range" });
        }
        if day < 1 || day > 31 {
            return Err(Self::Err { msg: "Day out of range" });
        }
        Ok(Self { year, month, day })
    }
}
