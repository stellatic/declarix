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
use colored::Colorize;
use regex::Regex;
use toml::Value;

use crate::manage_data::tools::{checker, convert_to_string, get_array, get_buffer};

use super::{database::database::{PackDatabase, PackStatements}, installers::{Arch, Builder, Debian, Fedora, Flatpak, OpenSUSE, Prog, Vsc}};

#[derive(Debug)]
pub enum Err {
    TooMany,
    InvalidPackage
}

pub enum Manager {
    Arch,
    Debian,
    OpenSUSE,
    Fedora,
    Vsc,
    Vscodium,
    Flatpak,
}

pub struct Install {
    pub gather: Vec<(String, Manager, Vec<Value>)>,
    pub arch: i32,
    pub vsc: i32
}

impl Install {
    pub fn new() -> Self {
        Self {
            gather: Vec::new(),
            arch: 0,
            vsc: 0,
        }
    }
    pub fn matches(&mut self, title: &str, installer: &Value) -> Result<(), Err> {
        let title = title.to_string();
        let array = get_array(&title, installer);
        match title.to_lowercase().as_str() {
            "paru" | "yay" | "pacman" => {
                self.gather.push((title,Manager::Arch, array));
                self.arch+=1;
            },
            "apt" => {
                self.gather.push((title, Manager::Debian, array))
            },
            "zypper" => {
                self.gather.push((title, Manager::OpenSUSE, array));
            },
            "dnf" => {
                self.gather.push((title, Manager::Fedora, array));
            },
            "vsc" | "code" | "vscode" => {
                self.gather.push(("code".to_string(),Manager::Vsc, array));
                self.vsc+=1
            },
            "vscodium" => {
                self.gather.push((title,Manager::Vscodium, array))
            }
            "flatpak" => {
                self.gather.push((title,Manager::Flatpak, array))
            }
            &_ => {
                Err(Err::InvalidPackage)?
            }
        }
        if self.arch > 1 || self.vsc > 1 {
            Err(Err::TooMany)?
        }
        Ok(())
    }

    pub fn structure(&mut self) {
        let db = PackDatabase::new();
        db.conn.execute("BEGIN TRANSACTION", ()).unwrap();
        db.create_table();
        let mut statements = PackStatements::new(&db.conn);
        self.gather.retain(|(title, _,packages)|{
            match title.as_str() {
                "paru" | "yay" | "pacman" => {
                    Arch::new(title).prog.init(packages, &mut statements);
                    false
                },
                "apt" => {
                    Debian::new(&title).prog.init(packages, &mut statements);
                    false
                },
                "zypper" => {
                    OpenSUSE::new(&title).prog.init(packages, &mut statements);
                    false
                },
                "dnf" => {
                    Fedora::new(&title).prog.init(packages, &mut statements);
                    false
                },
                &_ => {true}
            }
        });
        for (title,_,packages) in &self.gather {
            match title.as_str() {
                "code" => { Vsc::new(&title).prog.init(&packages, &mut statements) },
                "flatpak" => { Flatpak::new(&title).prog.init(&packages, &mut statements) }
                "vscodium" => { Vsc::new(&title).prog.init(&packages, &mut statements) }
                &_ => {}
            }
        }
        db.conn.execute("COMMIT TRANSACTION", ()).unwrap();
    }
}

impl Prog {
    fn init(&mut self, packages: &Vec<Value>, statements: &mut PackStatements) {
        let bin = PathBuf::from(format!("/usr/bin/{}",self.prog));
        if bin.exists() {
            let installed = format!("{:?}",checker(&self.prog, &self.checker));
            self.packages = convert_to_string(packages);
            let mut to_install = Vec::new();
            for package in &self.packages {
                if !self.installed(&installed, package) {
                    if self.prog == "code" || self.prog == "vscodium" {
                        self.install_command(&vec![package.to_string()]);
                    } else {
                        to_install.push(package.to_string());
                    }
                }
                if statements.update.execute((package, &self.prog)).unwrap() == 0 {
                    statements.insert.execute((package, &self.prog)).unwrap();
                }
            }
            if !to_install.is_empty() {
                self.install_command(&to_install);
            }
            self.uninstall(statements);
        }
        else {
            println!("{}",format!("{} is not installed on your system.",self.prog).red())
        }
    }

    fn uninstall(&self, statements: &mut PackStatements) {
        let pack_iter = statements.select.query_map([&self.prog],|row|{
            let value:String = row.get(0)?;
            Ok(value)
        }).unwrap();
        let mut to_uninstall = Vec::new();
        for package in pack_iter {
            if self.prog == "code" || self.prog == "vscodium" {
                self.uninstall_command(&vec![package.unwrap()]);
            } else {
                to_uninstall.push(package.unwrap())
            }
            
        }
        if !to_uninstall.is_empty() {
            self.uninstall_command(&to_uninstall);
        }
        statements.remove.execute([&self.prog]).unwrap();
        statements.zero.execute([&self.prog]).unwrap();
    }

    fn install_command(&self, prog: &Vec<String>) {
        get_buffer(&self.prog, &self.install, prog)
    }

    fn uninstall_command(&self, prog: &Vec<String>) {
        get_buffer(&self.prog, &self.uninstall, prog)
    }

    fn installed(&self, installed: &str, mtch: &str) -> bool {
        let reg = "\\\"|\\\\n|\\\\t| |/|.";
        let reg = &format!("({}|^){}({}|$)",&reg,&mtch,&reg);
        let re = Regex::new(reg).unwrap();
        return re.is_match(installed);
    }
}