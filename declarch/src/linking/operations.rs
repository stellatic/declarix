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
use crate::{removal::select::{Key, Path as Poth}, structures::structs::Link};
use std::{ffi::OsStr, fs::{self, Metadata}, io, os::unix::fs::{symlink, MetadataExt}, path::{Path, PathBuf}, process::{exit, Command}, time::UNIX_EPOCH};
use users::{get_current_gid, get_current_uid};
use shared::{copy_file, Ops};



pub trait Operation {
    fn get_met<T: AsRef<Path>>(&self, path: T) -> Metadata {
        match fs::metadata(&path) {
            Ok(meta) => meta,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                fs::metadata(&path.as_ref().parent().unwrap()).unwrap()
            }, 
            Err(err) => {
                println!("{}: {}",path.as_ref().display(),err);
                exit(1)
            }
        }
    }
    fn run_command(&self, op: Ops, arg: Vec<impl AsRef<OsStr>>) {
        Command::new("sudo").arg("declarchRoot").arg(op.to_string()).args(arg).output().unwrap();
    }

    fn check_perms(&self) -> bool;
    fn source_met(&self) -> Metadata;
    fn dest_met(&self) -> Metadata;
    fn operations(&self, op: Ops, args: Vec<impl AsRef<Path> + AsRef<OsStr>>) -> Result<(), std::io::Error> {
        if self.check_perms() {
            match op {
                Ops::Symlink => {
                    symlink(&args[0], &args[1])?;
                },
                Ops::Copy => {
                    copy_file(&args[0], &args[1])?
                },
                Ops::Create_Dir => {
                    fs::create_dir(&args[0])?;
                },
                Ops::Create_Dir_All => {
                    fs::create_dir_all(&args[0])?;
                },
                Ops::Hardlink => {
                    fs::hard_link(&args[0], &args[1])?;
                },
                Ops::Rm_Dir => {
                    fs::remove_dir(&args[0])?;
                },
                Ops::Rm_File => {
                    fs::remove_file(&args[0])?;
                }
            }
        } else {
            self.run_command(op, args)
        }
        Ok(())
    }
    fn get_nanos(&self, path: &PathBuf) -> i64 {
        fs::metadata(path).unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_nanos() as i64
    }
}

macro_rules! impl_operations {
    ($struct_name:ident) => {
        impl Operation for $struct_name {
            fn source_met(&self) -> Metadata {
                self.get_met(&self.source)
            }
    
            fn dest_met(&self) -> Metadata {
                self.get_met(&self.destination)
            }
    
            fn check_perms(&self) -> bool {
                let source = &self.source_met();
                source.uid() == get_current_uid() && source.gid() == get_current_gid()
            }
        }
    }
}


impl_operations!(Key);
impl_operations!(Poth);
impl_operations!(Link);

impl Link {

    pub fn copy_file(&self) -> Result<(), io::Error> {
        self.operations(Ops::Copy, vec![&self.source, &self.destination])?;
        Ok(())
    }

    pub fn create_dir(&self) -> Result<(), io::Error> {
        self.operations(Ops::Create_Dir, vec![&self.destination])?;
        Ok(())
    }

    pub fn hard_link(&self) -> Result<(), io::Error> {
        self.operations(Ops::Hardlink, vec![&self.source, &self.destination])?;
        Ok(())
    }

    fn _create_dir_all(&self) -> Result<(), io::Error> {
        self.operations(Ops::Create_Dir_All, vec![&self.source])?;
        Ok(())
    }
}