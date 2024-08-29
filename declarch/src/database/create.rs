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