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
use std::{io, fs, path::Path, str::FromStr, fmt::Display};

use filetime::{set_file_mtime, FileTime};

pub fn copy_file<T:AsRef<Path>>(source: &T, destination: &T) -> Result<(), io::Error> {
    fs::copy(source, &destination)?;
    let met = fs::metadata(source).unwrap();
    let met = FileTime::from_last_modification_time(&met);
    set_file_mtime(&destination, met)?;
    Ok(())
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Ops {
    Copy,
    Symlink,
    Create_Dir,
    Create_Dir_All,
    Hardlink,
    Rm_File,
    Rm_Dir
}

impl FromStr for Ops {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
        "Copy" => Ok(Ops::Copy),
        "Symlink" => Ok(Ops::Symlink),
        "Create_Dir" => Ok(Ops::Create_Dir),
        "Create_Dir_All" => Ok(Ops::Create_Dir_All),
        "Hardlink" => Ok(Ops::Hardlink),
        "Rm_Dir" => Ok(Ops::Rm_Dir),
        "Rm_File" => Ok(Ops::Rm_File),
        _ => Err(()),
        }
    }
}

impl Display for Ops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}