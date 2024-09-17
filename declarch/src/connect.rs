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
use std::{env::args, fmt::Formatter, fs::{self}, path::PathBuf, process::exit, str::FromStr};
use textwrap::{fill, Options};
use term_size::dimensions;
use colored::Colorize;
use toml::{Table, Value};

use crate::{database::database::{Database, PreparedStatements}, installation::init::{Err, Install}, manage_data::tools::get_table, services::services::Service, structures::structs::{Construct, Set, Setting}};


pub struct Connect {
    config_file: (bool,Option<String>),
    conf: PathBuf,
    link: (bool, Vec<String>),
    install: (bool, Vec<String>),
    service: (bool, Vec<String>),
    force: Force,
    mode: Mode,
    pub vec: (Vec<String>, bool, Set)
}

enum Force {
    Confirm,
    NoConfirm,
    None
}

enum OptionError<'a> {
    InvalidOption(&'a str),
    NoConfigPath,
    InvalidConfigPath(&'a str),
    ConfigNotExist(&'a PathBuf)
}

impl <'a>fmt::Display for OptionError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:","Error".red())?;
        match self {
            Self::InvalidOption(option) => {
                writeln!(f, "\tInvalid option: {}", option.red())?;
                writeln!(f,"\tUse `{}` for a list of correct options.","-h".blue())?
            },
            Self::NoConfigPath => {
                writeln!(f, "\tA config path was not specified.")?;
            },
            Self::InvalidConfigPath(option) => {
                writeln!(f,"Invalid config path {}",option.red())?;
                writeln!(f, "The config path must end with {}",".toml".blue())?
            },
            Self::ConfigNotExist(option) => {
                writeln!(f, "Provided config path does not exist: {}", option.display().to_string())?
            }
        }
        exit(1);
    }
}

enum Mode {
    Link,
    Install,
    Config,
    Service,
    None,
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Title {
    Config,
    Special_Config,
    Secure_Config,
    Secure_Special_Config,
    System,
    Special_System,
    Secure_System,
    Secure_Special_System,
    Backup,
    Secure_Backup,
    None,
}
impl Title {
    fn into_iter() -> Vec<Self> {
        vec![
            Title::Config,
            Title::Special_Config,
            Title::Secure_Config,
            Title::Secure_Special_Config,
            Title::System,
            Title::Special_System,
            Title::Secure_System,
            Title::Secure_Special_System,
            Title::Backup,
            Title::Secure_Backup,
            ]
    }
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}",self)?;
        Ok(())
    }
}
impl FromStr for Title {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "config" => Ok(Title::Config),
            "special_config" => Ok(Title::Special_Config),
            "secure_config" => Ok(Title::Secure_Config),
            "secure_special_config" => Ok(Title::Secure_Special_Config),
            "system" => Ok(Title::System),
            "special_system" => Ok(Title::Special_System),
            "secure_system" => Ok(Title::Secure_System),
            "secure_special_system" => Ok(Title::Secure_Special_System),
            "backup" => Ok(Title::Backup),
            "secure_backup" => Ok(Title::Secure_Backup),
            &_ => {Err("Invalid".to_string())}
        }
    }
}

impl Connect {
    pub fn new() -> Self {
        Self {
            config_file: (false, None),
            link: (false, Vec::new()),
            install: (false, Vec::new()),
            force: Force::None,
            mode: Mode::None,
            conf: PathBuf::from("/etc/declarch/declarch.toml"),
            vec: (Vec::new(), true, Set::None),
            service: (false, Vec::new())
        }
    }

    pub fn everything(&mut self) -> Result<(), Err> {
        let args: Vec<String> = args().collect();
        if args.get(1).is_some() {
            for arg in &args[1..] {
                if arg.starts_with("-") {
                    match arg.as_str() {
                        "-h" | "--help" => {
                            println!("{}",self.help_text())
                        },
                        "-c" | "--config" => {
                            self.config_file.0 = true;
                            self.mode = Mode::Config;
                        },
                        "-s" | "--service" => {
                            self.service.0 = true;
                            self.mode = Mode::Service;
                        }
                        "-l" | "--link" => {
                            self.link.0 = true;
                            self.mode = Mode::Link;
                        }
                        "-i" | "--install" => {
                            self.install.0 = true;
                            self.mode = Mode::Install;
                        },
                        "-f" | "--force" => {
                            self.force = Force::Confirm;
                        }
                        "-fc" | "--force-noconfirm" => {
                            if matches!(self.force, Force::None) {
                                self.force = Force::NoConfirm;
                            }
                        }
                        &_ => {
                            println!("{}",OptionError::InvalidOption(arg))
                        }
                    }
                } else {
                    match self.mode {
                        Mode::Config => {
                            if arg.ends_with(".toml") {
                                self.config_file.1 = Some(arg.to_string());
                            } else {
                                println!("{}",OptionError::InvalidConfigPath(arg));
                            }
                        },
                        Mode::Link => {
                            self.link.1.push(arg.to_string().to_lowercase());
                        },
                        Mode::Install => {
                            self.install.1.push(arg.to_string().to_lowercase());
                        },
                        Mode::Service => {
                            self.service.1.push(arg.to_string().to_lowercase());
                        },
                        Mode::None => {
                            println!("{}",OptionError::InvalidOption(arg))
                        }
                    }
                }
            }
            if self.config_file.0 {
                if let Some(config_path) = &self.config_file.1 {
                    self.conf = PathBuf::from(config_path);
                    if !self.conf.exists() {
                        println!("{}",OptionError::ConfigNotExist(&self.conf));
                    }
                } else {
                    println!("{}",OptionError::NoConfigPath);
                }
            }
            let conf = fs::read_to_string(&self.conf).unwrap();
            let conf:Table = toml::from_str(&conf).unwrap();
            if self.link.0 {
                let db = Database::new();
                let mut statements = PreparedStatements::new(&db.conn);
                db.conn.execute("BEGIN TRANSACTION", ()).unwrap();
                let aliases = self.get_alias(&conf);
                let paths = conf.get("paths");
                let config_path = conf.get("locations");
                let mut construct = Construct::new();
                if self.link.1.is_empty() {
                    for title in Title::into_iter() {
                        self.paths_process(title, &config_path, &mut statements, &aliases, paths, &mut construct);
                    }
                } else {
                    for dir in self.link.1.clone() {
                        match Title::from_str(&dir) {
                            Ok(title) => self.paths_process(title, &config_path, &mut statements, &aliases, paths, &mut construct),
                            Err(_err) => {
                                
                            }
                        }
                    }
                }
                db.conn.execute("COMMIT TRANSACTION", ()).unwrap();
                self.vec = construct.vec;
            }
            if self.install.0 {
                let installation = conf.get("install");
                let mut installer = Install::new();
                if let Some(installation) = installation {
                    for (title, inst) in get_table("install", installation) {
                        if self.install.1.is_empty() || self.install.1.contains(&title.to_lowercase()) {
                            installer.matches(&title, &inst)?
                        }
                    }
                    installer.structure()
                }
            }

            if self.service.0 {
                let service = Service::new();
                let services = conf.get("services");
                if let Some(services) = services {
                    for (title, s) in get_table("services", services) {
                        if self.service.1.is_empty() || self.service.1.contains(&title.to_lowercase()) {
                            service.match_service(&title, &s);
                        }
                    }
                }
            }
        } else {
            let conf = fs::read_to_string(&self.conf).unwrap();
            let conf:Table = toml::from_str(&conf).unwrap();
            let db = Database::new();
            let mut statements = PreparedStatements::new(&db.conn);
            db.conn.execute("BEGIN TRANSACTION", ()).unwrap();
            let aliases = self.get_alias(&conf);
            let paths = conf.get("paths");
            let config_path = conf.get("locations");
            let mut construct = Construct::new();
            for title in Title::into_iter() {
                self.paths_process(title, &config_path, &mut statements, &aliases, paths, &mut construct)
            }
            db.conn.execute("COMMIT TRANSACTION", ()).unwrap();
            self.vec = construct.vec;

            let mut installer = Install::new();
            if let Some(install) = conf.get("install") {
                for (title, inst) in get_table("install", &install) {
                    installer.matches(&title, &inst)?
                }
                installer.structure()
            }

            let services = Service::new();
            if let Some(service) = conf.get("services") {
                for (title, serv) in get_table("services", &service) {
                    services.match_service(&title, &serv)
                }
            }
        }
        Ok(())
    }
    fn paths_process(&mut self, title: Title, config_path: &Option<&Value>, statements: &mut PreparedStatements, aliases: &Table, paths: Option<&Value>, construct: &mut Construct) {
        match &title {
            Title::Config | Title::Secure_Config => {
                construct.set(title.clone(), Setting::Link).process_config(config_path, statements);
            },
            Title::Special_Config | Title::Secure_Special_Config => {
                construct.set(title.clone(), Setting::Special).process_config(&config_path, statements);
            },
            Title::System | Title::Secure_System => {
                construct.set(title.clone(), Setting::Link).process_paths(&config_path, aliases,&paths, statements);
            },
            Title::Special_System | Title::Secure_Special_System => {
                construct.set(title.clone(), Setting::Special).process_paths(&config_path, aliases, &paths, statements);
            },
            Title::Backup | Title::Secure_Backup => {
                construct.set(title.clone(), Setting::Copy).process_paths(&config_path, aliases, &paths, statements);
            },
            Title::None => {}
        }
    }

    fn help_text(&self) -> String {
        let help_text = "
Declarch
A declarative system management tool for various platforms. 
Usage:
    declarch [Options] [Sub-Options] <COMMANDS>

Options: 
    -h, --help          Show this help message

    -c, --config        Declare config location
                        (Default: /etc/declarch/declarch.toml)
            
    -l, --link          Only links based on provided config 
    (Options Explained under \"Link\")
            
    -i, --install       Only installs based on provided config
    (Options explained under \"Install\")

    -s, --services      Only enables/disables based on provided config
    (Options explained under \"Service\")
            
Link:
    -l, --link <list of paths>      
        Alone will link everything provided in your config paths
                                    
        Providing a list of paths allows you to select which paths to link
    
    Example:
        declarch -l config backup special_config

Install:
    -i, --install <list of managers> 
        
        Alone will install everything provided in your config
        
        Providing a list of managers allows you to select which managers to use.
        
    Example:
        declarch -i vsc flatpak paru

Service:
    -s, --services <list of service managers>
        
        Alone will use all service managers provided in your config

        Providing a list of service managers allows you to select which managers to use.

    Example:
        declarch -s systemd
        ";

        let terminal_width = dimensions().map(|w|w.0).unwrap_or(80 as usize);

        let options = Options::new(terminal_width).break_words(false);
        fill(&help_text, options)

    }
}