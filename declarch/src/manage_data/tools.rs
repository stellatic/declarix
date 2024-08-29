use core::fmt;
use std::{hash::{DefaultHasher, Hash, Hasher}, process::exit};
use colored::Colorize;
use toml::{map::Map, Value};

enum TomlError<'a> {
    InvalidTable(&'a str),
    InvalidArray(&'a str),
    InvalidString(&'a str),
}

impl <'a>fmt::Display for TomlError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}","Error".red())?;
        match self {
            Self::InvalidTable(a) => {
                writeln!(f, "Invalid Syntax for {}.", a.red())?;
                writeln!(f, "Correct syntax is: {}", format!("[[{a}]]").yellow())?
            },
            Self::InvalidArray(a) => {
                writeln!(f, "Invalid Syntax For: {}",a.red())?;
                writeln!(f, "Expected an array: {}","[\"a\", \"b\"]".yellow())?
            },
            Self::InvalidString(a) => {
                writeln!(f, "Invalid Syntax For:\n{}",a.red())?;
                writeln!(f, "Expected a string. Example: {}","\"example\"")?
            }
        }
        Ok(())
    }
}



pub fn get_table(title: &str, value: &Value) -> Map<String, Value> {
    if let Value::Table(value) = value {
        value.clone()
    } else {
        println!("{}",TomlError::InvalidTable(title));
        exit(1)
    }
}

pub fn get_array(title: &str,value: &Value) -> Vec<Value> {
    if let Value::Array(value) = value {
        value.clone()
    } else {
        println!("{}",TomlError::InvalidArray(title));
        exit(1);
    }
}

pub fn get_string(value: &Value) -> String {
    if let Value::String(value) = value {
        value.to_string()
    } else {
        println!("{}",TomlError::InvalidString(&format!("{:?}",value)));
        exit(1)
    }
}

pub fn fixer(path: &str) -> String {
    format!("/{}", path.trim_end_matches("/")
                        .trim_start_matches("/"))
}

pub fn calculate_hash(source: &str, destination: &str) -> u64 {
    let mut hasher =  DefaultHasher::new();
    source.hash(&mut hasher);
    destination.hash(&mut hasher);
    hasher.finish()
}