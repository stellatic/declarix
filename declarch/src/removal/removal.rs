use crate::{connect::Title, structures::structs::Setting};

use super::database::Removal;

impl <'conn>Removal<'conn> {
    pub fn removal(&mut self, setting: &Setting, title: &Title) {
        self.key_select(setting, title).unwrap();
    }
}