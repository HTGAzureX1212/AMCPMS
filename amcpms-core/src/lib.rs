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

use std::panic;

use backtrace::Backtrace;

pub use fltk;
use fltk::app::screen_size;
use fltk::dialog::message_title_default;
use fltk::dialog::alert;

pub fn install_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let payload = if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
            payload.to_string()
        } else if let Some(payload) = panic_info.payload().downcast_ref::<String>() {
            payload.clone()
        } else {
            String::from("unknown panic payload")
        };

        let backtrace = Backtrace::new();

        let message = format!("Unexpected panic occurred: {payload}\n
A bug report would be appreciated: https://github.com/HTGAzureX1212/AMCPMS/issues/new

Backtrace:
{backtrace:?}
");
        message_title_default("Application Error - Unexpected Panic");
        alert((screen_size().0 / 2.0) as i32, (screen_size().1 / 2.0) as i32, &message)
    }))
}
