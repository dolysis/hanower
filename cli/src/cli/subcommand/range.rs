use super::*;

use std::fmt;

#[derive(Debug, StructOpt)]
pub struct Range {
    #[structopt(long, default_value = "0")]
    start: i64,

    #[structopt(long, default_value = "256")]
    end: i64,

    #[structopt(long, default_value = "2")]
    count: u64,

    #[structopt(parse(try_from_str = NumberRange::parse_str))]
    range: Vec<NumberRange>,
}

impl Runner for Range {
    type Config = Options;

    fn run(
        &mut self,
        dst: &mut dyn std::io::Write,
        _config: Option<Self::Config>,
    ) -> Result<(), color_eyre::eyre::Report> {
        writeln!(
            dst,
            "start = {}, end = {}, count = {}",
            self.start, self.end, self.count
        )?;

        let interval = core::Interval::new(self.start as f64, self.end as f64, self.count)?;
        let mut numbers = self.range.iter();

        let mut range = None;
        if let Some(first) = numbers.next() {
            match first {
                NumberRange::Range { start, end } => range = Some((start, end)),
                NumberRange::Number(_) => numbers = self.range.iter(),
            }
        }

        let (low, high) = match range {
            Some((start, end)) => {
                let iter = interval
                    .iter()
                    .skip(*start as usize)
                    .take((*end - *start) as usize)
                    .map(|f| f.round() as i64);

                let start = iter.clone().min().unwrap_or(self.start);
                let end = iter.clone().max().unwrap_or(self.end);

                (start, end)
            }
            None => {
                let iter = interval.iter().map(|f| f.round() as i64);

                let start = iter.clone().min().unwrap_or(self.start);
                let end = iter.clone().max().unwrap_or(self.end);

                (start, end)
            }
        };

        let numbers = numbers
            .map(|n| match n {
                NumberRange::Number(num) => Ok(*num),
                NumberRange::Range { .. } => Err(NumberRangeError::UnexpectedRange),
            })
            .collect::<Result<Vec<i64>, _>>()?;

        write!(dst, "Matching numbers: ")?;
        for number in numbers {
            if low < number && high > number {
                write!(dst, "{} ", number)?;
            }
        }
        writeln!(dst)?;

        Ok(())
    }
}

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
