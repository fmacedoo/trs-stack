use rusqlite::{Connection, Result, Error};

use crate::models::todo::Todo;

pub struct TodosRepository<'a> {
    connection: &'a Connection,
}

impl TodosRepository<'_> {
    pub fn new(connection: &Connection) -> TodosRepository {
        TodosRepository {
            connection: connection
        }
    }

    pub fn query_all(&mut self) -> Result<Vec<Todo>, Error> {    
        let mut stmt = self.connection
            .prepare("SELECT * FROM todo",)?;

        let query = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    description: row.get(1)?,
                })
            })?;

        let mut result:Vec<Todo> = Vec::new();
        for todo in query {
            result.push(todo?)
        }

        Ok(result)
    }   
}
