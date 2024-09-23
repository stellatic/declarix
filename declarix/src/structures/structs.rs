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
use std::{fmt::Display, path::PathBuf, process::exit};
use serde_derive::{Deserialize,Serialize};

use crate::connect::Title;

#[derive(Clone, Debug)]
pub struct Construct {
    pub source: String,
    pub destination: String,
    pub title: Title,
    pub setting: Setting,
    pub source_path: String,
    pub destination_path: String,
    pub path: String,
    pub spec_src: String,
    pub spec_dec: String,
    pub hash: u64,
    pub set: Set,
    pub linker: Vec<Link>,
    pub vec: (Vec<String>, bool, Set)
}

#[derive(Debug, Clone)]
pub enum Set {
    Home,
    Root,
    Other,
    Generic,
    Default,
    None
}

impl Set {
    pub fn new(set: &str) -> Self {
        match set.to_lowercase().as_str() {
            "home" => Self::Home,
            "root" => Self::Root,
            "other" => Self::Other,
            "default" => Self::Default,
            "generic" => Self::Generic,
            &_ => {
                //ToDo Error
                exit(1);
            }
        }
    }
}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Setting {
    Link,
    Recursive,
    Copy,
    Secure_Link,
    Secure_Recursive,
    None,
}

impl Setting {
    pub fn into_iter() -> Vec<Self> {
        vec![
            Self::Link,
            Self::Recursive,
            Self::Secure_Link,
            Self::Secure_Recursive
        ]
    }
}

impl Display for Setting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}",self)
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Construct {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            destination: String::new(),
            path: String::new(),
            spec_src: String::new(),
            spec_dec: String::new(),
            hash: 0,
            title: Title::None,
            setting: Setting::None,
            source_path: String::new(),
            destination_path: String::new(),
            set: Set::Default,
            linker: Vec::new(),
            vec: (Vec::new(), true, Set::None)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Link {
    pub hash: u64,
    pub source: PathBuf,
    pub setting: Setting,
    pub title: Title,
    pub destination: PathBuf,
    pub special_source: String,
    pub set: Set,
    pub order: i64,
    pub vec: (Vec<String>, bool, Set)
}

impl Link {
    pub fn new(construct: &Construct, order: i64) -> Self {
        Self {
            hash: construct.hash,
            title: construct.title.clone(),
            source: PathBuf::from(construct.source.to_string()),
            destination: PathBuf::from(construct.destination.to_string()),
            setting: construct.setting.clone(),
            special_source: construct.path.to_string(),
            set: construct.set.clone(),
            order: order.clone(),
            vec: construct.vec.clone()
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Modified {
    pub source: String,
    pub destination: String,
    pub modified: i64,
}