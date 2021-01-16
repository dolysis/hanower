/*
use std::fmt;

#[derive(Debug, Clone)]
pub enum NumberRange {
    Number(i64),
    Range { start: i64, end: i64 },
}

impl NumberRange {
    fn parse_str(s: &str) -> Result<Self, NumberRangeError> {
        if let Some((start, end)) = parse_range(s) {
            return Ok(Self::Range { start, end });
        }

        s.parse::<i64>()
            .map(|i| Self::Number(i))
            .map_err(|_| NumberRangeError::InvalidNumber(s.to_string()))
    }
}

fn parse_range(s: &str) -> Option<(i64, i64)> {
    let mut inclusive = None;

    if s.split("..").count() == 2 {
        inclusive = Some(false)
    }

    if s.split("..=").count() == 2 {
        inclusive = Some(true)
    }

    let mut range = match inclusive? {
        true => s.split("..="),
        false => s.split(".."),
    };

    let start = range.next()?.parse::<i64>().ok()?;
    let mut end = range.next()?.parse::<i64>().ok()?;

    if inclusive? {
        end += 1;
    }

    Some((start, end))
}

#[derive(Debug)]
pub enum NumberRangeError {
    InvalidNumber(String),
    UnexpectedRange,
}

impl fmt::Display for NumberRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumber(ref invalid) => {
                write!(f, "Unable to parse as an integer: {}", invalid)
            }
            Self::UnexpectedRange => write!(f, "Found a range where a number was expected"),
        }
    }
}

impl std::error::Error for NumberRangeError {}
*/
