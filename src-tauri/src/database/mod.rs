pub mod todos;

use rusqlite::{Connection, Result};

use self::todos::TodosRepository;

pub struct Repository {
    connection: Connection,
}

impl Repository {
    pub fn new(connection: Connection) -> Repository {
        Repository {
            connection: connection
        }
    }

    pub fn todos(&mut self) -> TodosRepository {
        TodosRepository::new(&self.connection)
    }
}

pub fn connect() -> Result<Repository, rusqlite::Error> {
    let connection =
        Connection::open("todos.db")?;
    
    Ok(Repository::new(connection))
}
