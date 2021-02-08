/*
 * This Source Code Form is subject to the terms of
 * the Mozilla Public License, v. 2.0. If a copy of
 * the MPL was not distributed with this file, You
 * can obtain one at http://mozilla.org/MPL/2.0/.
 */

mod subcommand;

// use core::Interval;

use crate::{config::Options, run::Runner};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Root {
    #[structopt(subcommand)]
    cmd: Command,
}

impl Root {
    pub fn new() -> Self {
        Self::from_args()
    }

    pub fn execute(&mut self) -> Result<(), color_eyre::eyre::Report> {
        let cfg = Options::V1;
        let mut output = std::io::stdout();

        self.cmd.run(&mut output, Some(cfg))
    }
}

#[derive(Debug, StructOpt)]
enum Command {
    Range(subcommand::Range),
    Interval(subcommand::SubComInterval),
}

impl Runner for Command {
    type Config = Options;

    fn run(
        &mut self,
        dst: &mut dyn std::io::Write,
        config: Option<Self::Config>,
    ) -> Result<(), color_eyre::eyre::Report> {
        match self {
            Self::Range(cmd) => cmd.run(dst, config),
            Self::Interval(cmd) => cmd.run(dst, config),
        }
    }
}
