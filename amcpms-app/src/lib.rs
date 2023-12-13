/*
 * AMCPMS: Athletic Meet Computer Processing and Management System
 * Copyright (C) 2023 HTGAzureX1212.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use amcpms_core::fltk::app::{App, Scheme};
use amcpms_core::fltk::prelude::FltkError;
use amcpms_core::install_panic_hook;

pub struct AmcpmsApp {
    inner: App,
}

impl AmcpmsApp {
    pub fn new() -> Self {
        install_panic_hook();

        let app = App::default().with_scheme(Scheme::Base);
        Self {
            inner: app,
        }
    }

    pub fn run(self) -> Result<(), FltkError> {
        self.inner.run()
    }
}
