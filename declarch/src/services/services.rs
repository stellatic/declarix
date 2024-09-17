use core::fmt;
use std::path::PathBuf;

use colored::Colorize;
use regex::Regex;
use toml::Value;

use crate::manage_data::tools::{checker, convert_to_string, get_array, get_buffer};

use super::database::{ServiceDatabase, ServiceStatements};

#[derive(Debug)]
enum ServicesT {
    Systemd,
    SystemdUser
}

impl fmt::Display for ServicesT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Services {
    pub title: String,
    pub manager: String,
    pub stype: String,
    pub enable: Vec<String>,
    pub disable: Vec<String>,
    pub list: Vec<String>,

}

impl Services {
    fn new<'a>(manager: &'a str, title: ServicesT, stype: &'a str, enable: impl IntoIterator<Item = &'a str>, disable: impl IntoIterator<Item = &'a str>, list: impl IntoIterator<Item = &'a str>) -> Self {
        Self {
            title: title.to_string(),
            manager: manager.to_string(),
            stype: stype.to_string(),
            enable: enable.into_iter().map(String::from).collect(),
            disable: disable.into_iter().map(String::from).collect(),
            list: list.into_iter().map(String::from).collect(),
        }
    }
    fn enable(&self, prog: &Vec<String>) {
        get_buffer(&self.manager, &self.enable, prog)
    }

    fn disable(&self, prog: &Vec<String>) {
        get_buffer(&self.manager, &self.disable, prog);
    }

    fn enabler (&self, name: &str, stype: &Value, statements: &mut ServiceStatements) {
        if let Some(table) = stype.get(&self.stype) {
            if PathBuf::from(format!("/usr/bin/{}",name)).exists() {
                let check = format!("{:?}",checker(&self.manager, &self.list));
                let services = convert_to_string(&get_array(&self.stype, &table));
                let mut to_enable = Vec::new();
                for service in services {
                    if !self.enabled(&check, &service) {
                        to_enable.push(service.to_string());
                    }
                    if statements.update.execute((&self.title, &service)).unwrap() == 0 {
                        statements.insert.execute((&self.title, &service)).unwrap();
                    }
                }
                if !to_enable.is_empty() {
                    self.enable(&to_enable);
                }
                self.disabler(statements);
            } else {
                println!("{}",format!("{} is not installed on your system.",name).red())
            }
        }
    }
    fn enabled(&self, enabled: &str, mtch: &str) -> bool {
        let reg = "\\\\n|.service";
        let reg = &format!("({}|^){}({}|$)",&reg,&mtch,&reg);
        let re = Regex::new(reg).unwrap();
        return re.is_match(enabled);
    }

    fn disabler(&self, statements: &mut ServiceStatements) {
        let pack_iter = statements.select.query_map([&self.title],|row|{
            let value:String = row.get(0)?;
            Ok(value)
        }).unwrap();
        let mut to_disable = Vec::new();
        for service in pack_iter {
            to_disable.push(service.unwrap());
        }
        if !to_disable.is_empty() {
            self.disable(&to_disable);
        }
        statements.remove.execute([&self.title]).unwrap();
        statements.zero.execute([&self.title]).unwrap();
    }
}

pub struct Systemd {
    service: Services,
}

pub struct SystemdUser {
    service: Services,
}

pub trait Builder {
    fn new() -> Self;
}

impl Builder for Systemd {
    fn new() -> Self {
        let service = "systemctl";
        Self {
            service: Services::new("sudo",
            ServicesT::Systemd,
            "root",
            [service, "enable"], 
            [service, "disable"],
            [service, "list-unit-files", "--state=enabled"])
        }
    }
}

impl Builder for SystemdUser {
    fn new() -> Self {
        Self {
            service: Services::new("systemctl", 
            ServicesT::SystemdUser,
            "user",
            ["--user", "enable"],
            ["--user", "disable"],
            ["list-unit-files", "--user", "--state=enabled"])
        }
    }
}

pub struct Service {

}

impl Service {
    pub fn new() -> Self {
        Self {}
    }
    pub fn match_service(&self, title: &str, stype: &Value) {
        let db = ServiceDatabase::new();
        db.conn.execute("BEGIN TRANSACTION", ()).unwrap();
        db.create_table();
        let mut statements = ServiceStatements::new(&db.conn);
        let title = title.to_lowercase();
        match title.as_str() {
            "systemd" => {
                SystemdUser::new().service.enabler("systemctl", stype, &mut statements);
                Systemd::new().service.enabler("systemctl", stype, &mut statements);
            },
            &_ => {

            }
        }
        db.conn.execute("COMMIT TRANSACTION", ()).unwrap();
    }
}