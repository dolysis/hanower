/*
 * This Source Code Form is subject to the terms of
 * the Mozilla Public License, v. 2.0. If a copy of
 * the MPL was not distributed with this file, You
 * can obtain one at http://mozilla.org/MPL/2.0/.
 */

use super::*;
use structopt::clap::AppSettings::AllowLeadingHyphen;

#[derive(Debug, StructOpt)]
#[structopt(setting = AllowLeadingHyphen)]
pub struct SubComInterval {
    #[structopt(long, default_value = "2")]
    count: u64,

    #[structopt(allow_hyphen_values = true)]
    low: i64,

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

        let interval = core::Interval::new(low, high, self.count)?;
        let output: Vec<f64> = interval.intervals().collect();

        for &number in &output {
            write!(dst, "{} ", number.round())?;
        }
        writeln!(dst)?;

        Ok(())
    }
}
