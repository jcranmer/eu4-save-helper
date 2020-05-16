/// A date, in Paradox terms, consists of a 4-digit year. The calendar doesn't
/// follow any strict modern scheme, as leap years do not exist. The date after
/// Feburary 28th is always March 1, unlike in the Julian or Gregorian
/// calendars.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8
}

const MONTH_DAYS : [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.year, self.month, self.day)
    }
}

impl std::fmt::Debug for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
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

impl std::error::Error for ParseDateError {
}

impl std::fmt::Debug for ParseDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
        let s = s.trim_end();
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
        if day < 1 || day > MONTH_DAYS[(month - 1) as usize] {
            return Err(Self::Err { msg: "Day out of range" });
        }
        Ok(Self { year, month, day })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_parse() {
        fn to_date(s: &str) -> Result<Date, ParseDateError> {
            s.parse()
        }
        assert!(to_date("0.1.1").is_err());
        assert!(to_date("1.1.1").unwrap() == Date { year: 1, month: 1, day: 1});
        assert!(to_date("1444.2.28").unwrap() == Date { year: 1444, month: 2, day: 28});
        assert!(to_date("1444.2.29").is_err());
        assert!(to_date("1500.2.29").is_err());
    }
}
