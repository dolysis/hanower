/*
 * This Source Code Form is subject to the terms of
 * the Mozilla Public License, v. 2.0. If a copy of
 * the MPL was not distributed with this file, You
 * can obtain one at http://mozilla.org/MPL/2.0/.
 */

use hanower::Interval;

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
        interval.bucket(item as f64).and_then(|bucket| {
            buckets
                .get_mut(bucket)
                .filter(|current| select(item, **current))
                .map(|current| *current = Some(item))
        });
    }

    #[allow(clippy::filter_map_identity)]
    buckets.into_iter().filter_map(|opt| opt).collect()
}
