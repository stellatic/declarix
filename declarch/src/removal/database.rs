use rusqlite::{Connection, Error, Statement};
use super::select::Select;
use crate::database::database::{Keys, PrimaryPool, SecondaryPool, StatementPool};

// impl StatementPool {
//     pub fn zero_path<'a>(&self, title: &str) -> String {
//         format!(
//             "UPDATE Primary
//             SET to_keep = 0 
//             WHERE to_keep = 1 AND title = ?1"
//         )
//     }

//     pub fn delete(&self, title: &str) -> String {
//         format!(
//             "DELETE FROM {title}
//             WHERE to_keep = 0"
//         )
//     }
// }

pub trait Zero {
    fn zero(&self) -> String;
}

impl Zero for PrimaryPool {
    fn zero(&self) -> String {
        format!(
            "UPDATE Prime
            SET to_keep = 0
            WHERE hash = 1 AND title = ?1"
        )
    }
}

impl Zero for SecondaryPool {
    fn zero(&self) -> String {
        format!(
            "UPDATE Secondary
            SET to_keep = 0
            WHERE to_keep = 1 AND hash = ?1"
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
            WHERE to_keep = 0 AND title = ?1"
        )
    }
}

impl Delete for SecondaryPool {
    fn delete(&self) -> String {
        format!(
            "DELETE FROM Secondary
            WHERE to_keep = 0 AND hash = ?1"
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

// impl <'conn>Keys<'conn> {
//     fn select(conn: &'conn Connection, pool: &StatementPool) -> Result<Keys<'conn>, Error> {
//         Ok(Keys::handle(pool, conn,
//             pool.key.select(&pool.get_key()),
//             pool.path.select(&pool.get_path()),
//             pool.link.select(&pool.get_link())
//     )?)
//     }
//     fn zero(conn: &'conn Connection, pool: &StatementPool) -> Result<Keys<'conn>,Error> {
//         Ok(Keys::handle(pool, conn,
//             pool.zero_path(&pool.get_key()),
//             pool.zero_path(&pool.get_path()),
//         pool.zero_path(&pool.get_link())
//     )?)
//     }
//     fn delete(conn: &'conn Connection, pool: &StatementPool) -> Result<Keys<'conn>,Error> {
//         Ok(Keys::handle(pool, conn,
//             pool.delete(&pool.get_key()),
//             pool.delete(&pool.get_path()),
//             pool.delete(&pool.get_link())
//         )?)
//     }
// }

// pub struct Removal<'conn> {
//     pub select: Keys<'conn>,
//     pub link_select: Keys<'conn>,
//     pub zero: Keys<'conn>,
//     pub delete: Keys<'conn>
// }



// impl<'conn>Removal<'conn> {
//     pub fn new(conn: &'conn Connection, pool: &StatementPool) -> Result<Self, Error> {
//         Ok(Self {
//             select: Keys::select(conn, pool)?,
//             zero: Keys::zero(conn, pool)?,
//             delete: Keys::delete(conn, pool)?
//         })
//     }
// }