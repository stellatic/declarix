use std::{fs, io};
use crate::{database::database::PreparedStatements, structures::structs::Link};
use super::{link::Color, operations::Operation};

impl Link {
    
    pub fn copier(&mut self, statements: &mut PreparedStatements) -> Result<(), io::Error> {
        let nanos = statements.get_modified(&self);
        if self.destination.exists() {
            if self.source.is_file() {
                self.copy(statements, nanos.unwrap())?
            }
        } else {
            if self.source.is_file() {
                if self.source.is_symlink() {
                    let original = fs::canonicalize(&self.source).unwrap();
                    self.operations(shared::Ops::Symlink, vec![&original, &self.destination])?

                } else {
                    self.copy_file()?;
                    let dest_modified = self.get_nanos(&self.destination);
                    if nanos.unwrap().is_some() {
                        statements.update_modified(self, &dest_modified).unwrap()
                    } else {
                        statements.insert_copy(&dest_modified, self)
                    }
                }
            } else {
                self.create_dir()?;
                statements.insert_dir(self);
            }
        }
        Ok(())
    }

    fn copy(&mut self, statements: &mut PreparedStatements, nanos: Option<i64>) -> Result<(), io::Error> {
        let dest_met = self.get_nanos(&self.destination);
        if let Some(nanos) = nanos {
            let source_met = self.get_nanos(&self.source);
            if nanos < dest_met && source_met != dest_met || source_met < dest_met {
                if source_met == dest_met {
                    statements.update_modified(self, &nanos).unwrap();
                    self.set_vec(&Color::Green)
                } else {
                    self.set_vec(&Color::Red)
                }
            } else if source_met > dest_met {
                self.copy_file()?;
                let nanos = self.get_nanos(&self.destination);
                statements.update_modified(self, &nanos).unwrap();
            }
        } else {
            statements.insert_copy(&dest_met, self);
            self.set_vec(&Color::Blue);
        }
        Ok(())
    }
}