use std::{io::{BufRead, BufReader}, process::{Command, Stdio}};

use regex::Regex;
use toml::Value;

use crate::manage_data::tools::{get_array, get_string};

use super::{database::database::{PackDatabase, PackStatements}, installers::{Arch, Builder, Flatpak, Prog, Vsc}};

pub enum Err {
    TooMany,
    InvalidPackage
}

pub enum Manager {
    Arch,
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
            }
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
                }
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
        let installed = format!("{:?}",self.checker());
        self.packages = self.convert_to_string(packages);
        for package in &self.packages {
            if !self.installed(&installed, package) {
                self.install_command(package)
            }
            if statements.update.execute((package, &self.prog)).unwrap() == 0 {
                statements.insert.execute((package, &self.prog)).unwrap();
            }
        }
        self.uninstall(statements);

    }

    fn uninstall(&self, statements: &mut PackStatements) {
        let pack_iter = statements.select.query_map([&self.prog],|row|{
            let value:String = row.get(0)?;
            Ok(value)
        }).unwrap();
        for package in pack_iter {
            self.uninstall_command(&package.unwrap())
        }
        statements.remove.execute([&self.prog]).unwrap();
        statements.zero.execute([&self.prog]).unwrap();
    }

    fn install_command(&self, prog: &str) {
        self.get_buffer(&self.install, prog)
    }

    fn uninstall_command(&self, prog: &str) {
        self.get_buffer(&self.uninstall, prog)
    }

    fn installed(&self, installed: &str, mtch: &str) -> bool {
        let reg = "\\\"|\\\\n|\\\\t| ";
        let reg = &format!("({}|^){}({}|$)",&reg,&mtch,&reg);
        let re = Regex::new(reg).unwrap();
        return re.is_match(installed);
    }

    fn get_buffer(&self, args: &Vec<String>, prog: &str) {
        let stdout = self.command().args(args).arg(prog).stderr(Stdio::piped()).spawn().unwrap().stderr.unwrap();

        let reader = BufReader::new(stdout);

        reader.lines().filter_map(|line| line.ok()).for_each(|line| println!("{}",line))
    }

    fn checker(&self) -> String {
        String::from_utf8(self.command().args(&self.checker).stdout(Stdio::piped()).output().unwrap().stdout).unwrap()
    }

    fn command(&self) -> Command {
        Command::new(&self.prog)
    }

    fn convert_to_string(&self, values: &Vec<Value>) -> Vec<String> {
        values.iter().map(|value|{
            get_string(value)
        }).collect()
    }
}