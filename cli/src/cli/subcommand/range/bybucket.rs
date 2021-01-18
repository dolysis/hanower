// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

use core::Interval;

#[derive(Debug)]
pub(super) struct ByBucket {
    interval: Interval,
}

impl ByBucket {
    pub fn new(interval: Interval) -> Self {
        Self { interval }
    }

    pub fn select_max(&self, input: impl IntoIterator<Item = i64>) -> Vec<i64> {
        select_per_bucket(&self.interval, input, |new, current| {
            current.is_none() || current < Some(new)
        })
    }

    pub fn select_min(&self, input: impl IntoIterator<Item = i64>) -> Vec<i64> {
        select_per_bucket(&self.interval, input, |new, current| {
            current.is_none() || current > Some(new)
        })
    }
}

fn select_per_bucket<I, F>(interval: &Interval, input: I, select: F) -> Vec<i64>
where
    I: IntoIterator<Item = i64>,
    F: Fn(i64, Option<i64>) -> bool,
{
    let mut buckets: Vec<Option<i64>> = interval.iter().map(|_| None).collect();

    for item in input.into_iter() {
        bucket(interval, item as f64).and_then(|bucket| {
            buckets
                .get_mut(bucket)
                .filter(|current| select(item, **current))
                .map(|current| *current = Some(item))
        });
    }

    buckets.into_iter().filter_map(|opt| opt).collect()
}

fn bucket(interval: &Interval, number: f64) -> Option<usize> {
    if number < interval.low() || number >= interval.high() {
        return None;
    }

    let bucket = f64::ln(number - interval.low() + 1.0)
        / f64::ln(interval.high() - interval.low() + 1.0)
        * interval.count() as f64;

    Some(bucket.trunc() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket() {
        let data = vec![
            TestData::new(Interval::new(1.0, 10.0, 5).unwrap(), 8.0, Some(4)),
            TestData::new(Interval::new(30.0, 100.0, 10).unwrap(), 1000.0, None),
            TestData::new(Interval::new(30.0, 100.0, 10).unwrap(), 0.0, None),
            TestData::new(Interval::new(30.0, 100.0, 10).unwrap(), 10.0 * 10.0, None),
            TestData::new(Interval::new(-100.0, 100.0, 10).unwrap(), 0.0, Some(8)),
            TestData::new(Interval::new(-100.0, 100.0, 10).unwrap(), -100.0, Some(0)),
        ];

        for test in data {
            let actual = bucket(&test.interval, test.number);

            assert_eq!(test.expected, actual)
        }
    }

    struct TestData {
        interval: Interval,
        number: f64,
        expected: Option<usize>,
    }

    impl TestData {
        fn new(interval: Interval, number: f64, expected: Option<usize>) -> Self {
            Self {
                interval,
                number,
                expected,
            }
        }
    }
}
