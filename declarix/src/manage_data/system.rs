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
use dirs::home_dir;
use toml::{map::Map, Value};
use super::tools::{calculate_hash, get_array};
use crate::{database::database::PreparedStatements, structures::structs::{Construct, Link, Set,Setting}};

impl Construct {
    pub fn construct_system(&mut self, aliases: &Map<String, Value>, value: &Vec<Value>, statements: &mut PreparedStatements) {
        for value in value {
            let value = get_array(&self.title.to_string(), value);
            for (i, value) in value.iter().enumerate() {
                let mut path = self.process_alias(value, aliases);
                if i == 0 {
                    if ! matches!(self.set, Set::Generic) {
                        path = format!("{}{}",self.source_path, path);
                    }
                    self.source = path;
                } else {
                    if matches!(self.set, Set::Home) {
                        path = format!("{}{}",home_dir().unwrap().display(), path);
                    }
                    self.destination = path;
                    self.hash = calculate_hash(&self.source, &self.destination);
                    self.setting_match(statements);
                }
            }
        }
    }

    pub fn setting_match(&mut self, statements: &mut PreparedStatements) {
        match self.setting {
            Setting::Link | Setting::Secure_Link => {
                self.linker.push(Link::new(self, 0));
                statements.update_primary(self.hash).unwrap();
            },
            _ => {
                match self.get_special(statements) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("{}",e)
                    }
                }
            }
        }
    }
}