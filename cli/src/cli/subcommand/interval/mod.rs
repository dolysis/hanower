/*
 * This Source Code Form is subject to the terms of
 * the Mozilla Public License, v. 2.0. If a copy of
 * the MPL was not distributed with this file, You
 * can obtain one at http://mozilla.org/MPL/2.0/.
 */

use super::*;

#[derive(Debug, StructOpt)]
pub struct SubComInterval {
    /// To include negative ARGs values, put -- before the list of values{n}
    /// e.g. hanoi interval --count 8 -- -25 -4{n}
    #[structopt(default_value = "0")]
    start: i64,

    #[structopt(default_value = "256")]
    end: i64,

    #[structopt(long, default_value = "2")]
    count: u64,
}

impl Runner for SubComInterval {
    type Config = Options;

    fn run(
        &mut self,
        dst: &mut dyn std::io::Write,
        _config: Option<Self::Config>,
    ) -> Result<(), color_eyre::eyre::Report> {
        let interval = core::Interval::new(self.start as f64, self.end as f64, self.count)?;
        let output: Vec<f64> = interval.intervals().collect();

        for &number in &output {
            write!(dst, "{} ", number.round())?;
        }
        writeln!(dst)?;

        Ok(())
    }
}
