/*
 * This Source Code Form is subject to the terms of
 * the Mozilla Public License, v. 2.0. If a copy of
 * the MPL was not distributed with this file, You
 * can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::config::Options;

pub trait Runner {
    type Config: From<Options>;

    fn run(
        &mut self,
        dst: &mut dyn std::io::Write,
        config: Option<Self::Config>,
    ) -> Result<(), color_eyre::eyre::Report>;
}
