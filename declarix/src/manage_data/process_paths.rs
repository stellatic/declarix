use core::fmt;
use std::fmt::Display;
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
use std::fs;
use std::path::PathBuf;
use crate::database::database::PreparedStatements;
use crate::structures::structs::{Construct, Link, Set, Setting};
use colored::Colorize;
use dirs::config_dir;
use toml::{Table, Value};
use walkdir::WalkDir;

enum ConfigError<'a> {
    WrongSetting(&'a str, String)
}

impl <'a>Display for ConfigError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Error".red())?;
        match self {
            Self::WrongSetting(a, b) => {
                writeln!(f, "Invalid Title for: [{a}.{b}]")?
            }
        }
        Ok(())
    }
}

impl Setting {
    fn new(setting: &str) -> Result<Self, ConfigError> {
        match setting {
            "link" => Ok(Self::Link),
            "recursive" => Ok(Self::Recursive),
            "copy" => Ok(Self::Copy),
            &_ => {
                Err(ConfigError::WrongSetting("system", setting.to_string().to_lowercase()))
            }
        }
    }
}

use super::tools::{calculate_hash, fixer, get_array, get_string, get_table};
impl Construct {
    pub fn title_lower(&self) -> String {
        self.title.to_string().to_lowercase()
    }
    
    pub fn process_paths(&mut self, config_path: &Option<&Value>, paths: &Option<&Value>, aliases: &Table, statements: &mut PreparedStatements) {
        if let Some(system) = paths {
            for (title, value) in get_table("system", system) {
                let title = title.to_lowercase();
                self.setting = match Setting::new(&title) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("{}",e);
                        continue;
                    }
                };
                for (title, value) in get_table(&title, &value) {
                    let title = title.to_lowercase();
                    self.set = Set::new(&title);
                    self.source_path = self.get_locations(config_path, &title);
                    self.construct_system(aliases, &get_array(&title, &value), statements)
                }
                self.link_remove(statements);
            }
        }
    }

    pub fn process_config(&mut self, setting: Setting, config_path: &Option<&Value>, statements: &mut PreparedStatements) {
        self.setting = setting;
        let setting = self.setting.to_string().to_lowercase();
        self.source_path = self.get_locations(config_path, &setting);
        if PathBuf::from(self.source_path.clone()).exists() {
            self.destination_path = self.get_locations(config_path, "destination_config");
            for path in fs::read_dir(&self.source_path).unwrap() {
                let path = path.unwrap();
                let path = path.path().display().to_string();
                self.source = path.to_string();
                self.destination = format!("{}{}", self.destination_path,path.trim_start_matches(&self.source_path));
                self.hash = calculate_hash(&self.source, &self.destination);
                self.setting_match(statements);
            }
        }
        self.link_remove(statements);
        
    }
    pub fn get_locations(&self, config_path: &Option<&Value>, title: &str) -> String {
        let mut conf = "/etc/declarix".to_string();
        let config_path = config_path.and_then(|config|{
            if let Some(con) = config.get("directory") {
                conf = fixer(&get_string(con));
            }
            config.get(self.title_lower())
        }).and_then(|config| 
            config.get(title));
        if let Some(config) = config_path {
            fixer(&get_string(&config))
        } else {
            match title {
                "destination_config" => config_dir().unwrap().display().to_string(),
                &_ => format!("{}/{}/{}",conf,self.title_lower(),title)
            }
        }
    }

    fn link_remove(&mut self, statements: &mut PreparedStatements) {
        statements.remove.removal(&self.setting, &self.title);
        for link in &mut self.linker {
            match link.linker(statements) {
                Ok(a) => a,
                Err(err) => {
                    println!("{}: {}",link.source.display(),err)
                }
            }
            self.vec = link.vec.clone();
        }
        self.linker.clear();
    }

    pub fn set_path(&mut self, path: PathBuf) {
        let path = &path.display().to_string();
        self.path = path.trim_start_matches(&self.spec_src).to_string();
        self.source = path.clone();
        self.destination = format!("{}{}",&self.spec_dec,self.path);
    }
    pub fn get_special(&mut self, statements: &mut PreparedStatements) -> Result<(), SymlinkCheck> {
        self.spec_src = PathBuf::from(&self.source).parent().unwrap().display().to_string();
        self.spec_dec = PathBuf::from(&self.destination).parent().unwrap().display().to_string();
        let walkdir = WalkDir::new(&self.source);
        
        for (i, path) in walkdir.into_iter().enumerate() {
            self.set_path(path.unwrap().path().to_path_buf());
            if i == 0 {
                if PathBuf::from(&self.destination).is_symlink() {
                    return Err(SymlinkCheck::SymlinkError(self.destination.to_string()))
                }
                statements.key_insert_update(self).unwrap();
                statements.update_secondary(self, i as i64).unwrap();
                
            } else {
                statements.update_secondary(self, i as i64).unwrap();
            }
            self.linker.push(Link::new(self, i as i64));
        }
        Ok(())
    }
}

pub enum SymlinkCheck {
    SymlinkError(String)
}

impl fmt::Display for SymlinkCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}:","Error".red())?;
        match self {
            SymlinkCheck::SymlinkError(s) => {
                writeln!(f, "A symlink was found at {}.\nPlease rerun declarix for changes to be made.\nIf you see this error again, you will need to manually remove this symlink.", s.red())?
            }
        }
        Ok(())
    }
}