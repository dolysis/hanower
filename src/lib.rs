/*
 * This Source Code Form is subject to the terms of
 * the Mozilla Public License, v. 2.0. If a copy of
 * the MPL was not distributed with this file, You
 * can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! hanower is a CLI which calculates exponential backoffs from user input values.

//#![deny(missing_docs)]

use std::fmt;

/// Used to create and work with intervals which are calculated from the user-input CLI values.
///
/// - `low` is the starting point of the section from which to find intervals
/// - `high` is the inclusive end point of the section from which to find intervals
/// - `count` is the total number of desired intervals to be calculated
///    - must be a minimum of 2 (`low` and `high`)
#[derive(Debug, Clone, Copy)]
pub struct Interval {
    low: f64,
    high: f64,
    count: u64,
}

impl Interval {
    /// Creates a new Interval, with the range `low..=high`,
    /// split into `count` number of intervals.
    pub fn new(low: f64, high: f64, count: u64) -> Result<Self, IntervalError> {
        if low >= high {
            Err(IntervalError::InvalidRange)
        } else if count < 2 {
            Err(IntervalError::LowCount(count))
        } else {
            Ok(Self { low, high, count })
        }
    }

    /// Returns the `low` value.
    pub fn low(&self) -> f64 {
        self.low
    }

    /// Returns the `high` value.
    pub fn high(&self) -> f64 {
        self.high
    }

    /// Returns the `count` value.
    pub fn count(&self) -> u64 {
        self.count
    }

    // TODO: fix floating point accuracy bug
    /// Finds the bucket a given value exists in.
    ///
    /// A bucket refers to a range between two values, and including the starting value.
    ///
    /// For example, say we have values of `low = 1`, `high = 10`, and `count = 5`,
    /// and want to know which bucket the number `8` would be in. The output intervals
    /// would be `2 3 4 6 10`. The first bucket is then `2..<3`, next `3..<4`, etc.
    /// So, `8` is in the fourth bucket, between `6` and `10`.
    pub fn bucket(&self, number: f64) -> Option<usize> {
        if number < self.low() || number >= self.high() {
            return None;
        }

        let bucket = f64::ln(number - self.low() + 1.0) / f64::ln(self.high() - self.low() + 1.0)
            * self.count() as f64;

        Some(dbg!(bucket).trunc() as usize)
    }

    // /// Iterates through a given list of numbers, and finds the appropriate
    // /// matching value from the vec of resultant interval values.
    // ///     - you can search for the first, last or average values that fit into a bucket
    // pub fn in_list(&self, mut list: Vec<i64>) -> Option<Vec<i64>> {
    //     // ensures buckets are in order
    //     // maybe remove this, add error handling for this to the arg itself (when arg is added)?
    //     list.sort();

    //     // --- calculating resultant interval values ---
    //     let mut interval_values: Vec<i64> = vec![];

    //     for number in self.intervals().map(|f| f.round() as i64) {
    //         interval_values.push(number);
    //     }

    //     // --- calculating which resultant values fit the specified requirements ---
    //     let in_buckets: Vec<i64> = vec![];

    //     // for value in list
    //     //  if value == list.last()
    //     //      return in_buckets
    //     //  else
    //     //

    //     if in_buckets == vec![] {
    //         None
    //     } else {
    //         Some(in_buckets)
    //     }
    // }

    /// Returns an iterator of lazily evaluated intervals, starting from this
    /// Interval's `low` value up to and including the `high` value.
    pub fn iter(&self) -> IntervalIter {
        self.new_iter()
    }

    /// Returns an iterator of lazily evaluated intervals based on the
    /// `low` and `high points` of this Interval, and skips the floor value.
    pub fn intervals(&self) -> IntervalIter {
        let mut iter = self.new_iter();

        // Skip the floor value
        iter.next();

        iter
    }

    fn new_iter(&self) -> IntervalIter {
        debug_assert!(self.low < self.high, "Low must be less than high");
        debug_assert!(self.count >= 2, "Interval count must be >= 2.");

        IntervalIter::new(self.low, self.high, self.count)
    }
}

/// An iterator version of the [`Interval`] struct, which calculates intervals from the user-input CLI values.
///
/// - `low` is the starting point of the section from which to find intervals
/// - `high` is the inclusive end point of the section from which to find intervals
/// - `count` is the total number of desired intervals to be calculated
///    - must be a minimum of 2 (`low` and `high`)
/// - `idx_front` and `idx_back` are used to keep track of where the iterator is
#[derive(Debug, Clone)]
pub struct IntervalIter {
    low: f64,
    high: f64,
    count: u64,

    // Used by next()
    idx_front: u64,
    // Used by next_back()
    idx_back: u64,
}

impl IntervalIter {
    fn new(low: f64, high: f64, count: u64) -> Self {
        Self {
            low,
            high,
            count,
            idx_front: 0,
            idx_back: 0,
        }
    }

    fn idx(&self) -> u64 {
        self.idx_front + self.idx_back
    }

    fn calculate_interval(&self, index: u64) -> f64 {
        // scales `high` value down according to `low` value
        // `low` must always move down to 1.0
        let nlog = (self.high - self.low + 1.0).ln() / self.count as f64;
        let expo = (nlog * index as f64).exp();

        expo + self.low - 1.0
    }
}

impl Iterator for IntervalIter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx() > self.count {
            return None;
        }

        match self.idx_front {
            0 => {
                self.idx_front += 1;

                Some(self.low)
            }
            index => {
                let interval = self.calculate_interval(index);
                self.idx_front += 1;

                Some(interval)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // Because we iterate over `low` *and* `count` number
        // of intervals we need to add one
        let len = (self.count + 1) - self.idx();
        let len = len as usize;

        (len, Some(len))
    }
}

impl DoubleEndedIterator for IntervalIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx() > self.count {
            return None;
        }

        match self.count.checked_sub(self.idx_back) {
            Some(0) => {
                self.idx_back += 1;

                Some(self.low)
            }
            Some(index) => {
                let interval = self.calculate_interval(index);
                self.idx_back += 1;

                Some(interval)
            }
            None => {
                self.idx_back += 1;

                None
            }
        }
    }
}

impl ExactSizeIterator for IntervalIter {}

impl std::iter::FusedIterator for IntervalIter {}

/// Error kinds for command line arguments.
#[derive(Debug)]
pub enum IntervalError {
    /// Occurs when the user provides a `count` value below 2.
    LowCount(u64),
    /// Occurs when the user gives a `low` value >= `high`.
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
            Self::InvalidRange => {
                write!(f, "Invalid range. Ensure `start` value is less than `end`")
            }
        }
    }
}

impl std::error::Error for IntervalError {}

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

    /* --- TESTS --- */

    #[test]
    /// Checks that the Interval struct correctly detects and refuses invalid input values.
    fn start_after_end_err() -> TestResult {
        let args = Interval::new(10.0, 1.0, 5);

        // Assert that bad inputs lead to an error
        assert!(args.is_err());

        Ok(())
    }

    #[test]
    /// Checks that the Interval struct correctly detects and refuses invalid count values
    fn count_less_than_two_err() -> TestResult {
        let args = Interval::new(1.0, 10.0, 1);

        // Assert that a bad count leads to an error
        assert!(args.is_err());

        Ok(())
    }

    #[test]
    /// Runs the program's computed intervals against a series of precomputed data sets,
    /// checking that all of the actual outputs match the expected values
    fn hanower_algorithm_iter() -> TestResult {
        // For each set of args and precomputed outputs
        for (args, expected_list) in test_data().into_iter() {
            // Generate the actual outputs
            let actual_list: Vec<f64> = args.intervals().collect();

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

    #[test]
    fn hanower_algorithm_iter_back() -> TestResult {
        for (interval, expected_list) in test_data().into_iter() {
            let actual_list: IntervalIter = interval.intervals();

            interval.intervals().rev().zip(expected_list.iter().rev()).enumerate().try_for_each(|(idx, (actual, &expected))| {
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

            assert_output_length(actual_list.len(), expected_list.len())?
        }

        Ok(())
    }

    // tests the bucket method of Interval
    #[test]
    fn interval_bucket_method() {
        let data = vec![
            BucketTestData::new(Interval::new(1.0, 10.0, 5).unwrap(), 7.0, Some(4)),
            BucketTestData::new(Interval::new(30.0, 100.0, 10).unwrap(), 1000.0, None),
            BucketTestData::new(Interval::new(30.0, 100.0, 10).unwrap(), 0.0, None),
            BucketTestData::new(Interval::new(30.0, 100.0, 10).unwrap(), 10.0 * 10.0, None),
            BucketTestData::new(Interval::new(-100.0, 100.0, 10).unwrap(), 0.0, Some(8)),
            BucketTestData::new(Interval::new(-100.0, 100.0, 10).unwrap(), -100.0, Some(0)),
        ];

        for test in data {
            let actual = Interval::bucket(&test.interval, test.number);

            assert_eq!(test.expected, actual)
        }
    }

    // #[test]
    // fn first_in_buckets() {
    //     let expected: Option<Vec<i64>> = Some(vec![24, 46, 67]);
    //     let interval = Interval::new(10.0, 100.0, 10).unwrap();
    //     let actual = interval.in_list(vec![20, 40, 100, 60]);

    //     assert_eq!(expected, actual)
    // }

    /* --- HELPER STRUCTS & IMPLEMENTATIONS --- */
    struct BucketTestData {
        interval: Interval,
        number: f64,
        expected: Option<usize>,
    }

    impl BucketTestData {
        fn new(interval: Interval, number: f64, expected: Option<usize>) -> Self {
            Self {
                interval,
                number,
                expected,
            }
        }
    }

    /* --- HELPER FUNCTIONS --- */

    fn assert_output_length(actual: usize, expected: usize) -> TestResult {
        if let false = actual == expected {
            let msg = format!("Expected {} fences, but received: {}", expected, actual);

            error!(msg);
        }

        Ok(())
    }

    // helper function for `hanower_algorithm_iter` test
    // unwrap helps validate that the input data (Interval::new) is correct for test
    fn test_data() -> Vec<(Interval, Vec<i64>)> {
        vec![
            (Interval::new(1.0, 16.0, 4).unwrap(), vec![2, 4, 8, 16]),
            (
                Interval::new(100.0, 1000.0, 15).unwrap(),
                vec![
                    101, 101, 103, 105, 109, 114, 123, 137, 158, 192, 246, 330, 463, 671, 1000,
                ],
            ),
            (
                Interval::new(3.0, 72.0, 9).unwrap(),
                vec![4, 5, 6, 9, 13, 19, 29, 46, 72],
            ),
            (Interval::new(-19.0, 12.0, 3).unwrap(), vec![-17, -10, 12]),
            (
                Interval::new(-11000.0, -1200.0, 16).unwrap(),
                vec![
                    -10999, -10998, -10995, -10991, -10983, -10970, -10945, -10902, -10825, -10689,
                    -10446, -10016, -9252, -7894, -5483, -1200,
                ],
            ),
        ]
    }
}
