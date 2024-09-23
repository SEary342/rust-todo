use rusqlite::{Connection, Result, params};

pub struct Todo {
    pub id: u32,
    pub item: String,
}

pub struct TodoList {
    conn: Connection,
}

impl TodoList {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                item TEXT NOT NULL
            )",
            [],
        )?;

        Ok(TodoList { conn })
    }

    // Create
    pub fn add_todo(&self, item: &str) -> Result<i32> {
        self.conn.execute(
            "INSERT INTO todos (item) VALUES (?1)",
            params![item],
        )?;

        Ok(self.conn.last_insert_rowid() as i32)
    }

    // Read
    pub fn get_todo(&self, id: i32) -> Result<Option<Todo>> {
        let mut stmt = self.conn.prepare("SELECT id, item FROM todos WHERE id = ?1")?;
        let todo_iter = stmt.query_map(params![id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                item: row.get(1)?,
            })
        })?;

        todo_iter.next().transpose()
    }

    pub fn get_all_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self.conn.prepare("SELECT id, item FROM todos")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                item: row.get(1)?,
            })
        })?;

        todo_iter.collect()
    }

    // Update
    pub fn update_todo(&self, id: i32, new_item: &str) -> Result<usize> {
        self.conn.execute(
            "UPDATE todos SET item = ?1 WHERE id = ?2",
            params![new_item, id],
        )
    }

    // Delete
    pub fn delete_todo(&self, id: i32) -> Result<usize> {
        self.conn.execute("DELETE FROM todos WHERE id = ?1", params![id])
    }
}
