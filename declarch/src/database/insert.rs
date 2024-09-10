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
use rusqlite::{Connection, Error};

use crate::structures::structs::{Construct, Link};

use super::database::{Keys, PreparedStatements, PrimaryPool, SecondaryPool, StatementPool};

pub trait Insert {
    fn insert(&self) -> String;
}

impl Insert for PrimaryPool {
    fn insert(&self) -> String {
        format!(
            "INSERT INTO Prime (hash, category, title, source, destination, to_keep)
                VALUES (?1, ?2, ?3, ?4, ?5, 1)
                ;")
    }
}

impl Insert for SecondaryPool {
    fn insert(&self) -> String {
        format!(
            "INSERT INTO Secondary (hash, path, modified, path_order, to_keep)
                VALUES (?1, ?2, ?3, ?4, 0)
                ;")
    }
}
impl <'conn>Keys<'conn> {
    pub fn insert(conn: &'conn Connection, pool: &StatementPool) -> Result<Self, Error> {
        Ok(Self {
            primary: conn.prepare(&pool.primary.insert())?,
            secondary: conn.prepare(&pool.secondary.insert())?,
        })
    }
}

impl <'conn>PreparedStatements<'conn> {
    pub fn key_insert(&mut self, construct: &Construct) -> Result<(), Error> {
        self.insert.primary.execute((construct.hash as i64, &construct.set.to_string(), &construct.title.to_string(), &construct.spec_src, &construct.spec_dec))?;
        Ok(())
    }

    pub fn link_insert(&mut self, link: &Link) -> Result<(), Error> {
        self.insert.primary.execute((link.hash as i64, &link.set.to_string(), &link.title.to_string(), &link.source.display().to_string(), &link.destination.display().to_string()))?;
        Ok(())
    }
}