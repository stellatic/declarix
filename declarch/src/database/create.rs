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
use super::database::{PrimaryPool, SecondaryPool};

pub trait Create {
    fn create<'a>(&self) -> String;
}

impl Create for PrimaryPool {
    fn create<'a>(&self) -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS Prime (
                hash INTEGER NOT NULL,
                category TEXT NOT NULL,
                title TEXT NOT NULL,
                source TEXT NOT NULL,
                destination TEXT NOT NULL,
                to_keep BOOL NOT NULL
                );")
    }
}

impl Create for SecondaryPool {
    fn create<'a>(&self) -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS Secondary (
                hash INTEGER NOT NULL,
                path TEXT NOT NULL,
                modified INTEGER NOT NULL,
                path_order INTEGER NOT NULL,
                to_keep BOOL NOT NULL
                );")
    }
}