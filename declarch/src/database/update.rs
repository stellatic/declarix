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

pub trait Update {
    fn update(&self) -> String;
}

impl Update for PrimaryPool {
    fn update(&self) -> String {
        format!(
            "UPDATE Prime
            SET to_keep = 1
            WHERE hash = ?1
            ;")
    }
}

impl Update for SecondaryPool {
    fn update(&self) -> String {
        format!(
            "UPDATE Secondary
            SET to_keep = 1 AND path_order = ?1
            WHERE hash = ?2 AND path = ?3
            ;")
    }
}

impl <'conn>Keys<'conn> {
    pub fn update(conn: &'conn Connection, pool: &StatementPool) -> Result<Self, Error> {
        Ok(Self {
            primary: conn.prepare(&pool.primary.update())?,
            secondary: conn.prepare(&pool.secondary.update())?
        })
    }
}

impl <'conn> PreparedStatements<'conn> {
    pub fn key_insert_update(&mut self, construct: &mut Construct) -> Result<(), Error> {
        if self.update_primary(construct.hash)? == 0 {
            self.key_insert(construct).unwrap();
        }
        Ok(())
    }
    pub fn link_insert_update(&mut self, link: &Link) -> Result<(), Error> {
        if self.update_primary(link.hash)? == 0 {
            self.link_insert(link).unwrap();
        }
        Ok(())
    }

    pub fn update_primary(&mut self, id: u64) -> Result<usize, Error> {
        self.update.primary.execute([id as i64])
    }

    pub fn update_secondary(&mut self, construct: &Construct, order: i64) -> Result<(), Error> {
        self.update.secondary.execute((order, construct.hash as i64, &construct.path))?;
        Ok(())
    }
}