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
use core::fmt;
use std::{fs, io, path::PathBuf};

use colored::Colorize;
use rusqlite::{Error, Row};

use crate::{connect::Title, database::database::{PrimaryPool, SecondaryPool}, linking::operations::Operation, structures::structs::Setting};

use super::database::Removal;

enum OperationE {
    Error((String, io::Error))
}

impl fmt::Display for OperationE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationE::Error(a) => {
                writeln!(f, "{}:","Error".red())?;
                writeln!(f,"Problem removing: {}",a.0)?;
                writeln!(f,"{}",a.1)?
            }
        }
        Ok(())
    }
}

pub trait Select {
    fn select(&self) -> String;
}

impl Select for PrimaryPool {
    fn select(&self) -> String {
        format!("SELECT hash, source, destination, category, setting FROM Prime WHERE title = ?1 AND setting = ?2;")
    }
}

impl PrimaryPool {
    pub fn link_select(&self) -> String {
        format!("SELECT hash, source, destination, category, setting FROM Prime WHERE to_keep = 0 AND title = ?1 AND setting = ?2;")
    }
}

impl Select for SecondaryPool {
    fn select(&self) -> String {
        format!(
            "SELECT path, path_order, modified FROM Secondary
            WHERE hash = ?1 AND to_keep = 0
            ORDER BY path_order DESC
            ;")
    }
}
impl <'conn>Removal<'conn> {
    pub fn key_select(&mut self, setting: &Setting, title: &Title) {
        let title = title.to_string();
        let set = setting.to_string();
        let stmt = match setting {
            Setting::Link | Setting::Secure_Link => &mut self.link_select,
            _ => &mut self.select.primary
        };
        let rows: Result<Vec<Key>, Error> = stmt.query_map([&title, &set], |row| {
            let key = Key::new(row)?;
            Ok(key)
        }).unwrap().collect();
        match setting {
            Setting::Link | Setting::Secure_Link => self.link_remove(&rows.unwrap()),
            _ => self.paths_select(&rows.unwrap()).unwrap(),
        }
        self.delete.primary.execute([&title, &set]).unwrap();
        self.zero.primary.execute([&title, &set]).unwrap();
    }

    pub fn link_remove(&mut self, stmt: &Vec<Key>) {
        for path in stmt {
            match path.removal() {
                Ok(a) => a,
                Err(err) => {
                    println!("{}",OperationE::Error((path.destination.to_string(), err)))
                }
            }
        }
    }


    pub fn paths_select(&mut self, key_iter: &Vec<Key>) -> Result<(), Error> {
        for key in key_iter {
            let id = key.hash;
            let stmt = self.select.secondary.query_map([id], |row|{
                Ok({
                    let path: String = row.get(0)?;
                    Path {
                        source: PathBuf::from(format!("{}{}",key.source,path)),
                        destination: PathBuf::from(format!("{}{}",key.destination,path)),
                        order: row.get(1)?,
                        modified: row.get(2)?,
                        category: key.category.to_string()
                    }
                })
            }).unwrap();
            for path in stmt {
                let path = path?;
                match path.removal() {
                    Ok(a) => a,
                    Err(err) => {
                        println!("{}",OperationE::Error((path.destination.display().to_string(), err)))
                    }
                }
            }
            self.delete.secondary.execute([id]).unwrap();
            self.zero.secondary.execute([id]).unwrap();
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Key {
    pub hash: i64,
    pub source: String,
    pub destination: String,
    pub category: String,
    pub setting: String,
}

pub trait Remove {
    fn symlink_remove(&self, source: &str, destination: &PathBuf) -> bool {
        let source = PathBuf::from(source);
        let mut to_remove = false;
        match fs::canonicalize(&destination) {
            Ok(source_link) => {
                if source_link == source {
                    to_remove = true;
                }
            },
            Err(err) => {
                match err.kind() {
                    std::io::ErrorKind::NotFound => to_remove = true,
                    _ => {}
                }
            }
        }
        to_remove
    }
}

impl Remove for Key {}
impl Remove for Path {}

impl Key {
    fn new(row: &Row) -> Result<Self, Error> {
        Ok(Self {
            hash: row.get(0)?,
            source: row.get(1)?,
            destination: row.get(2)?,
            category: row.get(3)?,
            setting: row.get(4)?,
        })
    }
    fn removal(&self) -> Result<(),io::Error> {
        
        let destination = PathBuf::from(&self.destination);
        if destination.exists() || destination.is_symlink() {
            let mut to_remove = false;
            if destination.is_file() || destination.is_symlink() {
                if destination.is_symlink() {
                    to_remove = self.symlink_remove(&self.source, &destination);
                } else if destination.is_file() {
                    if self.get_met(&self.source).modified().unwrap() == self.get_met(&self.destination).modified().unwrap() {
                        to_remove = true;
                    }
                }
                if to_remove == true {
                    self.remove_file().unwrap()
                }
            }
        }
        Ok(())
    }

    fn remove_file(&self) -> Result<(), std::io::Error> {
        self.operations(shared::Ops::Rm_File, vec![&self.destination])
    }
}

impl Path {
    pub fn removal(&self) -> Result<(), io::Error> {
        let destination = &self.destination;
        if destination.exists() || destination.is_symlink() {
            if destination.is_file() || destination.is_symlink() {
                let mut to_remove = false;
                if destination.is_symlink() {
                    to_remove = self.symlink_remove(&self.source.display().to_string(), &self.destination)
                } else if self.modified == self.get_nanos(&self.destination) {
                    to_remove = true;
                }
                if to_remove == true {
                    self.remove_file()?
                }
            } else {
                self.remove_dir()?
            }
        }
        Ok(())
    }

    fn remove_file(&self) -> Result<(), std::io::Error> {
        self.operations(shared::Ops::Rm_File, vec![&self.destination])
    }

    fn remove_dir(&self) -> Result<(), std::io::Error> {
        self.operations(shared::Ops::Rm_Dir, vec![&self.destination])
    }
}

#[derive(Debug)]
pub struct Path {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub modified: i64,
    pub order: i64,
    pub category: String
}