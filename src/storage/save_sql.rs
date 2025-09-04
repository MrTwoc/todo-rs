use rusqlite::{Connection, Result};

// 初始化数据库
pub fn get_conn() -> Result<Connection> {
    // let conn = Connection::open_in_memory()?;
    let conn = Connection::open("todo-rs.db")?;
    Ok(conn)
}
