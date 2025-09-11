use rusqlite::{Connection, Result};
/*
    如果考虑到支持多人协同操作的话，可以将数据库换为：PgSql
    或者orm：
    https://rbatis.github.io/rbatis.io/#/v4/
*/
// 初始化数据库
pub fn get_conn() -> Result<Connection> {
    // let conn = Connection::open_in_memory()?;
    let conn = Connection::open("todo-rs.db")?;
    Ok(conn)
}
