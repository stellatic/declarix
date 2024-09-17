/*
Copyright (C) 2024  StarlightStargaze

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
mod structures;
mod manage_data;
use std::process::exit;

use connect::Connect;
mod linking;
pub mod installation;
mod database;
mod connect;
mod services;
pub mod removal;
use users::{get_current_gid, get_current_uid};
use colored::Colorize;

fn main() /*-> Result<(), Error>*/ {
    if get_current_gid() == 0 || get_current_uid() == 0 {
        println!(
"{}
Declarch already checks file permissions before linking/copying, and will request you to enter sudo.
After you are requested, Declarch has another binary called \"declarchRoot\" which handles files/directories with root permissions.
This binary is ran separately so Declarch itself never runs as root.
Running the whole program with sudo will cause file/directory permission issues, so it has been disabled.",
"Using Declarch with sudo is dangerous.".red());
    exit(1);
    }
    let mut connect = Connect::new();
    connect.everything();
    for a in connect.vec.0 {
        println!("{a}")
    }
}


