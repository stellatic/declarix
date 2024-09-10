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
use std::process::exit;

use colored::Colorize;
use dirs::home_dir;
use regex::Regex;
use regex_split::RegexSplit;
use toml::{map::Map, Table, Value};
use super::tools::{fixer, get_table};
use crate::{connect::Connect, structures::structs::Construct};
use super::tools::get_string;

enum AliasError {
    InvalidAlias(String),
    NoMatch(String)
}

impl fmt::Display for AliasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}:","Error".red())?;
        match self {
            AliasError::InvalidAlias(s) => {
                writeln!(f, "Invalid Alias: {}",s.red())?;
                writeln!(f,"Valid syntax for alias: {}, {}, {} or {}","[example]".yellow(),"{example}".yellow(),"(example)".yellow(),"{[(example}])".yellow())?
            },
            AliasError::NoMatch(s) => {
                writeln!(f, "No Matching Alias for: {}",s.red())?
            }
        }
        Ok(())
    }
}

impl Connect {
    pub fn get_alias(&self, table: &Table) -> Table {
        let mut aliases = Map::new();
        table.get("aliases").map(|a| {
            for (title, _) in get_table("aliases", a) {
                let r = Regex::new(r"^(\[|\{|\()(.*)(\]|\}|\))$").unwrap();
                if !r.is_match(&title) {
                    println!("{}",AliasError::InvalidAlias(title));
                }
            }
            aliases = get_table("aliases", a);
            aliases.insert("[home]".to_string(), Value::String(home_dir().unwrap().display().to_string()));
        });
        aliases
    }
}

impl Construct {
    pub fn process_alias(&self, value: &Value, aliases: &Map<String, Value>) -> String {
        let mut path = get_string(value);
        let r = Regex::new(r"^(\[|\{|\()(.*)(\]|\}|\))").unwrap();
        let split: Vec<&str> = r.split_inclusive(&path).collect();
        if split.len() > 1 {
            path = split.iter().map(|s|{
                if r.is_match(s) {
                    if let Some(alias) = aliases.get(*s) {
                        fixer(&get_string(&alias))
                    } else {
                        println!("{}",AliasError::NoMatch(s.to_string()));
                        exit(1);
                    }
                } else {
                    fixer(&s)
                }
            }).collect::<String>();
        } else {
            path = fixer(&path);
        }
        path
    }
}