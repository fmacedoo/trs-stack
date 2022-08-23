use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

#[derive(Debug)]
struct TodoRepository {
    conn: 
}

trait TodoRepository {
    fn new (){
        let conn = Connection::open("todos.db")?;
        Self
    };
}

fn main() -> Result<()> {
    let conn = Connection::open("todos.db")?;

    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        NO_PARAMS,
    )?;

    Ok(())
}