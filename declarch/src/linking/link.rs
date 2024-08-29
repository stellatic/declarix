use std::{fs, io, os::unix::fs::MetadataExt};
use colored::Colorize;
use rusqlite::Result;
use crate::{database::database::PreparedStatements, structures::structs::{ Link, Set,  Setting}};
use shared::Ops;

use super::operations::Operation;

impl Link {
    pub fn linker(&mut self, statements: &mut PreparedStatements) -> Result<(), io::Error> {
        match self.setting {
            Setting::Link => {
                self.link(statements)?;
            },
            Setting::Special => {
                self.special_link(statements)?;
            },
            Setting::Copy => {
                self.copier(statements)?;
            },
            Setting::None => {}
        }
        Ok(())
    }

    fn special_link(&mut self, statements: &mut PreparedStatements) -> Result<(), io::Error> {
        if !self.source.exists() {
            return Err(std::io::Error::new(io::ErrorKind::NotFound, format!("{}: Source Path Not Found: {}","Error".red(),self.source.display())))
        }
        if self.destination.exists() {
            if self.destination.is_file() || self.destination.is_symlink() {
                self.symlink_test();
            }
        } else {
            self.if_exists()?;
            if self.source.is_file() {
                self.symlink()?;
            } else {
                self.create_dir()?;
            }
            statements.special_insert(self).unwrap();
        }
        Ok(())
    }

    // pub fn create_dir(&self) {
    //     self.operations(Ops::Create_Dir, vec![&self.destination])
    // }

    fn link(&mut self, statements: &mut PreparedStatements) -> Result<(), io::Error> {
        if !self.source.exists() {
            return Err(std::io::Error::new(io::ErrorKind::NotFound, format!("{}: Source Path Not Found: {}","Error".red(),self.source.display())))
        }
        if self.destination.exists() {
            self.symlink_test();
        } else {
            self.if_exists()?;
            self.symlink()?;
            statements.link_insert_update(self).unwrap();
        }
        Ok(())
    }

    fn if_exists(&self) -> Result<(), io::Error> {
        if ! self.destination.exists() {
            if let Some(dest) = self.destination.parent() {
                if ! dest.exists() {
                    self.operations(Ops::Create_Dir_All, vec![&dest.to_path_buf()])?
                }
            } else {
                //ToDo Error
            }
        }
        Ok(())
    }

    fn symlink(&self) -> Result<(), io::Error> {
        if matches!(&self.set, Set::Root) {
            let (source, destination) = (self.get_met(&self.source.display().to_string()).dev(), self.get_met(self.destination.display().to_string()).dev());
            if source == destination {
                self.hard_link()?;
            } else {
                self.operations(Ops::Symlink, vec![&self.source, &self.destination])?
            }
        } else {
            self.operations(Ops::Symlink, vec![&self.source, &self.destination])?
        }
        Ok(())
    }

    fn symlink_test(&mut self) {
        let (source, destination) = (self.source_met(), self.dest_met());
        if matches!(self.set, Set::Root) && source.dev() == destination.dev() {
            if source.modified().unwrap() != destination.modified().unwrap() {
                self.set_vec(&Color::None)
            }
        } else {
            if fs::canonicalize(&self.destination).unwrap() != self.source {
                self.set_vec(&Color::None)
            }
        }
    }
    pub fn set_vec(&mut self, color: &Color) {
        let mut path = self.destination.display().to_string();
        match color {
            Color::Blue => {
                path = path.blue().to_string();
            },
            Color::Red => {
                path = path.red().to_string();
            },
            Color::Green => {
                path = path.green().to_string();
            }
            Color::None => {}
        }
        if self.vec.1 == true {
            self.vec.0.push(self.title.to_string().replace("_", " "));
            self.vec.1 = false;
            self.push_set();
        }
        self.push_set();
        self.vec.0.push(path);
    }
    fn push_set(&mut self) {
        if !matches!(self.set, Set::Default) {
            if self.set.to_string() != self.vec.2.to_string() {
                self.vec.0.push(self.set.to_string().underline().to_string());
                self.vec.2 = self.set.clone();
            }
        }
    }
}

pub enum Color {
    Blue,
    Red,
    Green,
    None
}