use dirs::home_dir;
use toml::{map::Map, Value};
use super::tools::{calculate_hash, fixer, get_array};
use crate::{database::database::PreparedStatements, structures::structs::{Construct, Link, Set,Setting}};

impl Construct {
    // pub fn process_set(&mut self, value: &Value, aliases: &Map<String, Value>, statements: &mut PreparedStatements) {
    //     for (title, value) in &get_table(value) {
    //         self.set = Set::new(title);
    //         self.construct_system(aliases, &get_array(value), statements)
    //     }
        
    // }

    pub fn construct_system(&mut self, aliases: &Map<String, Value>, value: &Vec<Value>, statements: &mut PreparedStatements) {
        for value in value {
            let value = get_array(&self.title.to_string(), value);
            for (i, value) in value.iter().enumerate() {
                let mut path = self.process_alias(value, aliases);
                if i == 0 {
                    if ! matches!(self.set, Set::Generic) {
                        let set = fixer(&self.set.to_string().to_lowercase());
                        path = format!("{}{}{}",self.source_path,set,path);
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
        if matches!(self.setting, Setting::Link) {
            self.linker.push(Link::new(self, 0));
            statements.update_primary(self.hash).unwrap();
        } else {
            self.get_special(statements);
        }

    }
}