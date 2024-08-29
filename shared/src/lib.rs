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