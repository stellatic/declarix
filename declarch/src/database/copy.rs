use rusqlite::{Connection, Error, Rows, Statement};

use crate::structures::structs::Link;

use super::database::{PreparedStatements, StatementPool};

impl StatementPool {
    pub fn select_modified(&self) -> String {
        format!(
            "SELECT modified FROM Secondary WHERE hash = ?1 AND path = ?2
            ;")
    }

    pub fn update_modified(&self) -> String {
        format!(
            "UPDATE Secondary
            SET modified = ?1
            WHERE hash = ?2 AND path = ?3
            ;")
    }
}

impl <'conn>PreparedStatements<'conn> {
    pub fn select_modified(&mut self, link: &Link) -> Result<Rows, Error> {
        Ok(self.copy.select_modified.query((link.hash as i64, &link.special_source))?)
    }

    pub fn get_modified(&mut self, link: &Link) -> Result<Option<i64>, Error> {
        let mut num: Option<i64> = None;
        if let Some(row) = self.select_modified(link)?.next()? {
            num = Some(row.get(0)?)
        }
        Ok(num)
    }
}

pub struct Copy<'conn> {
    pub select_modified: Statement<'conn>,
    pub update_modified: Statement<'conn>,
}

impl <'conn>Copy <'conn> {
    pub fn new(conn: &'conn Connection, pool: &StatementPool) -> Result<Self, Error> {
        Ok(Self {
            select_modified: conn.prepare(&pool.select_modified())?,
            update_modified: conn.prepare(&pool.update_modified())?
        })
    }
}