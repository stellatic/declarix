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
use std::fs;

use dirs::data_dir;
use rusqlite::{Connection, Statement};

pub struct PackDatabase {
    pub conn: Connection
}

impl PackDatabase {
    pub fn new() -> Self {
        let mut db = data_dir().unwrap().join("declarch");
        if ! db.exists() {
            fs::create_dir_all(&db).unwrap();
        }
        db = db.join("packages.db");

        Self { conn: Connection::open(db).unwrap() }
    }

    pub fn create_table(&self) {
        self.conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS Packages (
                package     TEXT NOT NULL,
                manager     TEXT NOT NULL,
                to_keep     BOOL
                );"
        ), ()).unwrap();
    }
}

pub struct PackStatements<'conn> {
    pub insert: Statement<'conn>,
    pub update: Statement<'conn>,
    pub select: Statement<'conn>,
    pub zero: Statement<'conn>,
    pub remove: Statement<'conn>,
}

impl <'conn>PackStatements<'conn> {
    pub fn new(conn: &'conn Connection) -> Self {
        let states = StatementPool::new();
        Self {
            insert: conn.prepare(&states.insert).unwrap(),
            update: conn.prepare(&states.update).unwrap(),
            select: conn.prepare(&states.select).unwrap(),
            zero: conn.prepare(&states.zero).unwrap(),
            remove: conn.prepare(&states.remove).unwrap()
        }
    }
}

struct StatementPool {
    insert: String,
    update: String,
    select: String,
    zero: String,
    remove: String,
}

impl StatementPool {
    fn new() -> Self {
        Self {
            update: format!(
                "UPDATE Packages
                    SET to_keep = 1
                    WHERE package = ?1 AND manager = ?2
                ;"),
            insert: format!(
                "INSERT INTO Packages (package, manager, to_keep)
                    VALUES (?1, ?2, 1)
                ;"),
            select: format!(
                "SELECT package
                FROM Packages
                WHERE manager = ?1 AND to_keep = 0
                ;"),
            zero: format!(
                "UPDATE Packages
                SET to_keep = 0
                WHERE to_keep = 1 AND manager = ?1
                ;"),
            remove: format!(
                "DELETE FROM Packages
                WHERE to_keep = 0 AND manager = ?1
                ;")

        }
    }
}