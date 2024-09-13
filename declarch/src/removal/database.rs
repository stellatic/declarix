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
use rusqlite::{Connection, Error, Statement};
use super::select::Select;
use crate::database::database::{Keys, PrimaryPool, SecondaryPool, StatementPool};

pub trait Zero {
    fn zero(&self) -> String;
}

impl Zero for PrimaryPool {
    fn zero(&self) -> String {
        format!(
            "UPDATE Prime
            SET to_keep = 0
            WHERE to_keep = 1 AND title = ?1;"
        )
    }
}

impl Zero for SecondaryPool {
    fn zero(&self) -> String {
        format!(
            "UPDATE Secondary
            SET to_keep = 0
            WHERE to_keep = 1 AND hash = ?1;"
        )
    }
}

pub trait Delete {
    fn delete(&self) -> String;
}

impl Delete for PrimaryPool {
    fn delete(&self) -> String {
        format!(
            "DELETE FROM Prime
            WHERE to_keep = 0 AND title = ?1;"
        )
    }
}

impl Delete for SecondaryPool {
    fn delete(&self) -> String {
        format!(
            "DELETE FROM Secondary
            WHERE to_keep = 0 AND hash = ?1;"
        )
    }
}


pub struct Removal<'conn> {
    pub select: Keys<'conn>,
    pub link_select: Statement<'conn>,
    pub zero: Keys<'conn>,
    pub delete: Keys<'conn>,
    pub test: Statement<'conn>
}

impl <'conn>Removal<'conn> {
    pub fn new(conn: &'conn Connection, pool: &StatementPool) -> Result<Self, Error> {
        Ok(Self {
            select: Keys::select(conn, pool)?,
            link_select: conn.prepare(&pool.primary.link_select())?,
            zero: Keys::zero(conn, pool)?,
            delete: Keys::delete(conn, pool)?,
            test: conn.prepare("SELECT to_keep FROM Prime WHERE to_keep = 0 AND title = ?1")?
        })
    }
}

impl <'conn>Keys<'conn> {
    fn select(conn: &'conn Connection, pool: &StatementPool) -> Result<Keys<'conn>, Error> {
        Ok(Self {
            primary: conn.prepare(&pool.primary.select())?,
            secondary: conn.prepare(&pool.secondary.select())?
        })
    }

    fn zero(conn: &'conn Connection, pool: &StatementPool) -> Result<Keys<'conn>, Error> {
        Ok(Self {
            primary: conn.prepare(&pool.primary.zero())?,
            secondary: conn.prepare(&pool.secondary.zero())?
        })
    }

    fn delete(conn: &'conn Connection, pool: &StatementPool) -> Result<Keys<'conn>, Error> {
        Ok(Self {
            primary: conn.prepare(&pool.primary.delete())?,
            secondary: conn.prepare(&pool.secondary.delete())?
        })
    }
}