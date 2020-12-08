// use csv;
// use std::collections::BinaryHeap;
use std::{fmt, io};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hanoi")]
struct Opts {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(default_value = "10.0")]
    start: f64,

    #[structopt(default_value = "100.0")]
    end: f64,

    #[structopt(default_value = "2")]
    number: u64,

    #[structopt(short, long)]
    file: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    low: f64,
    high: f64,
    count: u64,
}

impl Interval {
    /// Create a new Interval, with the range `low..=high`, split into `count`
    /// number of intervals
    pub fn new(low: f64, high: f64, count: u64) -> Result<Self, IntervalError> {
        if low >= high {
            return Err(IntervalError::InvalidRange);
        } else if count < 2 {
            return Err(IntervalError::LowCount(count));
        } else {
            Ok(Self { low, high, count })
        }
    }

    /// Calculate a set of intervals based on the low and high points of
    /// this Interval
    pub fn intervals(&self) -> Vec<f64> {
        debug_assert!(self.low < self.high, "Low must be less than high");
        debug_assert!(self.count >= 2, "Interval count must be >= 2.");

        if self.count == 2 {
            vec![self.low, self.high]
        } else {
            let mut intervals = Vec::new();

            // scale high value down according to low value
            // low must always move down to 1.0
            // let scaled_end = end - start + 1.0;
            let nlog = (self.high - self.low + 1.0).ln() / self.count as f64;

            // iterate over desired length of return vector (== count)
            // fill in the incremental fencepost values
            for idx in 1..=self.count {
                let expo = (nlog * idx as f64).exp();
                let post = expo + self.low - 1.0;
                intervals.push(post);
            }

            intervals
        }
    }
}

#[derive(Debug)]
pub enum IntervalError {
    /// The given count does not satisfy the requirements
    LowCount(u64),
    /// The given high and low points were invalid
    InvalidRange,
}

impl fmt::Display for IntervalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LowCount(bad) => write!(
                f,
                "Invalid count. Ensure `number` value is >= 2 (was: {})",
                bad
            ),
            Self::InvalidRange => write!(f, "Invalid range. Ensure start is less than end"),
        }
    }
}

impl std::error::Error for IntervalError {}

// uses find_fenceposts() vec to identify buckets
// and then pulls the index number & row data from the max index
// in those buckets
// fn bucket_maxes(fence_vec: Vec<f64>, file: String) -> Vec<f64> {
//     match read_csv(file) {
//         Err(e) => eprintln!("{}", e),
//         Ok(index_vec) => {
//             let _min = fence_vec[0];
//             let _max = fence_vec[(Opts::from_args().number as usize) - 1];
//             let _bucket_vec: Vec<f64> = vec![];

//             return index_vec; //placeholder return
//         }
//     }

//     return vec![0.0, 99.0]; //placeholder return
// }

// fn read_csv(path: String) -> Result<Vec<f64>, Box<dyn Error>> {
//     let mut reader = csv::Reader::from_path(path)?;
//     let mut index_vec = vec![0.0];

//     for result in reader.records() {
//         let record = result?;
//         let value: f64 = record[0].parse().unwrap();
//         index_vec.push(value);
//     }

//     Ok(index_vec)
// }

fn main() {
    let _opt = Opts::from_args();
    // println!("{:#?}", opt);

    // let fence_vec = find_fenceposts(opt.start, opt.end, opt.number);
    // bucket_maxes(fence_vec, opt.file);
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    // All functions (excluding main), structs and traits that are defined above
    // can be used in this module
    use super::*;
    use anyhow::{bail as error, Error as AnyError};

    /// Typedef of the Results our #[test] functions return
    type TestResult = std::result::Result<(), AnyError>;

    /*
     * For reference this is how I calculated the expected values in `test_data`:
     *
     *  -- INPUTS
     *      * $START  : The low value
     *      * $END    : The high value
     *      * $COUNT  : The number of fences to divide $START..=$END into
     *
     * -- OUTPUT
     *      * $OUTPUT : An array containing $COUNT fence points
     *
     * -- FUNCTIONS
     *      * ln      : See [f64::ln](https://doc.rust-lang.org/std/primitive.f64.html#method.ln)
     *      * exp     : See [f64::exp](https://doc.rust-lang.org/std/primitive.f64.html#method.ln)
     *
     * -- ALGORITHM
     * $CEILING := ln($END - $START + 1)
     * $NLOG    := $CEILING / $COUNT
     *
     * for $IDX in 1..=$COUNT:
     *      $EXP := exp($NLOG * $IDX)
     *      $POST    := $EXP + $START - 1
     *      $OUTPUT  += $POST
     *
     * return $OUTPUT
     */

    /// Small struct for carrying around fence function arguments
    #[derive(Debug, Clone, Copy)]
    struct FenceArgs {
        pub start: f64,
        pub end: f64,
        pub count: u64,
    }

    impl FenceArgs {
        fn new(start: f64, end: f64, count: u64) -> Self {
            Self { start, end, count }
        }
    }

    /// Wrapper function for passing around our actual function
    fn fence_fn(args: FenceArgs) -> Result<Vec<f64>, AnyError> {
        let your_fn = |start, end, count| {
            Interval::new(start, end, count)
                .map(|i| i.intervals())
                .map_err(Into::into)
        };

        your_fn(args.start, args.end, args.count)
    }

    /* --- TESTS --- */

    #[test]
    /// Checks that the fence function correctly detects and refuses invalid input values.
    fn start_after_end_err() -> TestResult {
        let args = FenceArgs::new(10.0, 1.0, 5);

        let test: Result<Vec<f64>, AnyError> = fence_fn(args);

        // Assert that bad inputs lead to an error
        assert!(test.is_err());

        Ok(())
    }

    #[test]
    /// Checks that the fence function correctly detects and refuses invalid count values
    fn count_less_than_two_err() -> TestResult {
        let args = FenceArgs::new(1.0, 10.0, 1);

        let test: Result<Vec<f64>, AnyError> = fence_fn(args);

        // Assert that a bad count leads to an error
        assert!(test.is_err());

        Ok(())
    }

    #[test]
    /// Runs the fence function against a series of precomputed data sets checking that
    /// all of the actual outputs match the expected values
    fn hanoi_algorithm_static_data() -> TestResult {
        let test_values = test_data().into_iter();

        // For each set of args and precomputed outputs
        for (args, expected_list) in test_values {
            // Generate the actual outputs
            let actual_list = fence_fn(args)?;

            // For each expected and actual data sets
            actual_list.iter().zip(expected_list.iter()).enumerate().try_for_each(|(idx, (&actual, &expected))| {
                // Round the actual item
                let rounded = actual.round() as i64;

                // Check that the rounded item matches the precomputed item
                if rounded != expected {
                    let msg = format!(
                        "@{} => Expected {}, received {} ({})\nExpected Values | {:?}\nActual Values  | {:?}",
                        idx, expected, rounded, actual, expected_list, actual_list
                    );

                    error!(msg);
                }

                Ok(())
            })?;

            // Assert the lists are the same length, if they aren't the actual data is wrong
            assert_output_length(actual_list.len(), expected_list.len())?
        }

        Ok(())
    }

    /* --- HELPER FUNCTIONS -- */

    fn assert_output_length(actual: usize, expected: usize) -> TestResult {
        if let false = actual == expected {
            let msg = format!("Expected {} fences, but received: {}", expected, actual);

            error!(msg);
        }

        Ok(())
    }

    fn test_data() -> Vec<(FenceArgs, Vec<i64>)> {
        vec![
            (FenceArgs::new(1.0, 16.0, 4), vec![2, 4, 8, 16]),
            (
                FenceArgs::new(100.0, 1000.0, 15),
                vec![
                    101, 101, 103, 105, 109, 114, 123, 137, 158, 192, 246, 330, 463, 671, 1000,
                ],
            ),
            (
                FenceArgs::new(3.0, 72.0, 9),
                vec![4, 5, 6, 9, 13, 19, 29, 46, 72],
            ),
            (FenceArgs::new(-19.0, 12.0, 3), vec![-17, -10, 12]),
            (
                FenceArgs::new(-11000.0, -1200.0, 16),
                vec![
                    -10999, -10998, -10995, -10991, -10983, -10970, -10945, -10902, -10825, -10689,
                    -10446, -10016, -9252, -7894, -5483, -1200,
                ],
            ),
        ]
    }
}
