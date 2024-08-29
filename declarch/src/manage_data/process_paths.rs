use std::fs;
use std::path::PathBuf;
use crate::database::database::PreparedStatements;
use crate::structures::structs::{Construct, Link, Set};
use dirs::config_dir;
use toml::{Table, Value};
use walkdir::WalkDir;

use super::tools::{calculate_hash, fixer, get_array, get_string, get_table};
impl Construct {
    pub fn title_lower(&self) -> String {
        self.title.to_string().to_lowercase()
    }

    pub fn process_paths(&mut self,config_path: &Option<&Value>, aliases: &Table, table: &Option<&Value>, statements: &mut PreparedStatements) {
        let title = self.title_lower();
        let path = table 
            .and_then(|paths| paths.get(&title));
        self.source_path = self.get_config(config_path, &title);
        match path {
            Some(path) => {
                let path = get_table(&title, path);
                for (title, value) in path {
                    self.set = Set::new(&title);
                    self.construct_system(aliases, &get_array(&title,&value), statements)
                }
                self.link_remove(statements);
            }, 
            None => {

            }
        }
    }

    pub fn process_config(&mut self, config_path: &Option<&Value>, statements: &mut PreparedStatements) {
        let title = self.title_lower();
        self.source_path = self.get_config(config_path, &title);
        self.destination_path = self.get_config(config_path, "destination_config");
        for path in fs::read_dir(&self.source_path).unwrap() {
            let path = path.unwrap();
            let path = path.path().display().to_string();
            self.source = path.to_string();
            self.destination = format!("{}{}", self.destination_path,path.trim_start_matches(&self.source_path));
            self.hash = calculate_hash(&self.source, &self.destination);
            self.setting_match(statements);
        }
        self.link_remove(statements);
        
    }
    pub fn get_config(&self, config_path: &Option<&Value>, title: &str) -> String {
        config_path.and_then(|config| config.get(title));
        if let Some(config) = config_path {
            fixer(&get_string(&config))
        } else {
            match title {
                "destination_config" => config_dir().unwrap().display().to_string(),
                &_ => format!("/etc/declarch/{}", title)
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
    }

    pub fn set_path(&mut self, path: PathBuf) {
        let path = &path.display().to_string();
        self.path = path.trim_start_matches(&self.spec_src).to_string();
        self.source = path.clone();
        self.destination = format!("{}{}",&self.spec_dec,self.path);
    }
    pub fn get_special(&mut self, statements: &mut PreparedStatements) {
        self.spec_src = PathBuf::from(&self.source).parent().unwrap().display().to_string();
        self.spec_dec = PathBuf::from(&self.destination).parent().unwrap().display().to_string();
        let walkdir = WalkDir::new(&self.source);
        
        for (i, path) in walkdir.into_iter().enumerate() {
            self.set_path(path.unwrap().path().to_path_buf());
            if i == 0 {
                statements.key_insert_update(self).unwrap();
                statements.update_secondary(self, i as i64).unwrap();
                
            } else {
                statements.update_secondary(self, i as i64).unwrap();
            }
            self.linker.push(Link::new(self, i as i64));
        }
    }
}