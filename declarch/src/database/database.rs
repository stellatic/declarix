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

    pub fn special_insert(&mut self, link: &Link) -> Result<(), Error> {
        self.insert.secondary.execute((link.hash as i64, &link.special_source, 0, &link.order))?;
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