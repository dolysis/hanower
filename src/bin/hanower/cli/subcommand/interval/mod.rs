/*
 * This Source Code Form is subject to the terms of
 * the Mozilla Public License, v. 2.0. If a copy of
 * the MPL was not distributed with this file, You
 * can obtain one at http://mozilla.org/MPL/2.0/.
 */

use super::*;
use structopt::clap::AppSettings::AllowLeadingHyphen;

/// Finds the `count` number of intervals in a range from `low` to `high`
#[derive(Debug, StructOpt)]
#[structopt(setting = AllowLeadingHyphen)]
pub struct SubComInterval {
    /// Number of intervals, minimum of 2
    #[structopt(long, default_value = "2")]
    count: u64,

    /// Start point of section from which to find intervals
    #[structopt(allow_hyphen_values = true)]
    low: i64,

    /// End point of section from which to find intervals
    #[structopt(allow_hyphen_values = true)]
    high: i64,
}

impl Runner for SubComInterval {
    type Config = Options;

    fn run(
        &mut self,
        dst: &mut dyn std::io::Write,
        _config: Option<Self::Config>,
    ) -> Result<(), color_eyre::eyre::Report> {
        let low = self.low as f64;
        let high = self.high as f64;

        let interval = hanower::Interval::new(low, high, self.count)?;

        //for number in interval.intervals().map(|f| f.round() as i64) {
        for number in interval.intervals() {
            let res = number.abs() - number.round().abs();
            // dbg!(res);
            // Work around for rust's atrocious floating point support
            if (res / number).abs() < 1e-9 {
                write!(dst, "{:.0} ", number.round())?
            } else {
                write!(dst, "{} ", number)?
            }
        }
        writeln!(dst)?;

        Ok(())
    }
}
