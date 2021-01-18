// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

mod cli;
mod config;
mod run;

use cli::Root;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    Root::new().execute()
}
