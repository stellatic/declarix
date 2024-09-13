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
use dirs::data_dir;
use rusqlite::{Connection, Error, Statement};

use crate::{removal::database::Removal, structures::structs::Link};

use super::{copy::Copy, create::Create};

pub struct Database {
    pub conn: Connection
}

impl Database {
    pub fn new() -> Self  {
        let conn = Connection::open(data_dir().unwrap().join("declarch").join("tracker.db")).unwrap();
        Self { conn }
    }
}
pub struct PrimaryPool {}

pub struct SecondaryPool {}

pub struct StatementPool {
    pub primary: PrimaryPool,
    pub secondary: SecondaryPool,

}

impl StatementPool {
    fn new() -> Self {
        Self {
            primary: PrimaryPool {},
            secondary: SecondaryPool {},
        }
    }
}

pub struct Keys <'conn> {
    pub primary: Statement<'conn>,
    pub secondary: Statement<'conn>,
}

pub struct PreparedStatements<'conn> {
    pub insert: Keys<'conn>,
    pub update: Keys<'conn>,
    pub copy: Copy<'conn>,
    pub remove: Removal<'conn>
}

impl <'conn>PreparedStatements <'conn> {
    pub fn new(conn: &'conn Connection) -> Self {
        let pool = StatementPool::new();
        conn.execute(&pool.primary.create(), ()).unwrap();
        conn.execute(&pool.secondary.create(), ()).unwrap();
        Self {
            insert: Keys::insert(conn, &pool).unwrap(),
            update: Keys::update(conn, &pool).unwrap(),
            copy: Copy::new(conn, &pool).unwrap(),
            remove: Removal::new(conn, &pool).unwrap()
        }
    }

    pub fn special_insert(&mut self, link: &Link, nanos: &i64) -> Result<(), Error> {
        self.insert.secondary.execute((link.hash as i64, &link.special_source, nanos, &link.order))?;
        Ok(())
    }

    pub fn insert_copy(&mut self, nanos: &i64, link: &Link) {
        self.insert.secondary.execute((link.hash as i64, &link.special_source, nanos, &link.order)).unwrap();
    }

    pub fn insert_dir(&mut self, link: &Link) {
        self.insert.secondary.execute((link.hash as i64,&link.special_source, 0, &link.order)).unwrap();
    }

    pub fn update_modified(&mut self, link: &Link, nanos: &i64) -> Result<(), Error> {
        self.copy.update_modified.execute((nanos, link.hash as i64, &link.special_source))?;
        Ok(())
    }
}