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
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fs::hard_link, os::unix::fs::symlink};
use shared::copy_file;
use shared::Ops;
fn main()  {
    let args: Vec<String> = env::args().collect();
    match Ops::from_str(&args[1]).unwrap() {
        Ops::Symlink => {
            symlink(&args[2], &args[3]).unwrap()
        },
        Ops::Hardlink => {
            hard_link(&args[2], &args[3]).unwrap();
        },
        Ops::Copy => {
            copy_file(&PathBuf::from(&args[2]), &PathBuf::from(&args[3])).unwrap();
        }
        Ops::Create_Dir => {
            std::fs::create_dir(&args[3]).unwrap();
        },
        Ops::Rm_Dir => {
            std::fs::remove_dir(&args[2]).unwrap();
        }, 
        Ops::Rm_File => {
            std::fs::remove_file(&args[2]).unwrap();
        }
        _ => {}
    }
}