use tracing::info;

use crate::storage::save_sql::get_conn;
use crate::user::user::User;

impl User {
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        let db_exists = std::path::Path::new("todo-rs.db").exists();
        if !db_exists {
            let conn = get_conn()?;

            conn.execute(
                "CREATE TABLE user(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                password TEXT,
                level INTEGER DEFAULT 0
                )",
                (),
            )?;
            info!("数据库和用户表创建成功 [1/2]");

            conn.execute_batch(
                "BEGIN;
         INSERT INTO user (name, password, level) VALUES (admin, admin, 1);
         INSERT INTO user (name, password, level) VALUES (user, user, 0);
         COMMIT;",
            )?;
            info!("测试用户插入成功 [2/2]");
            info!("用户初始化完成");
        }

        Ok(())
    }

    pub fn list() -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let conn = get_conn()?;
        let mut stmt = conn.prepare("SELECT id, name, password, level FROM user")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                password: row.get(2)?,
                level: row.get(3)?,
            })
        })?;

        let mut users = Vec::new();
        for user in user_iter {
            users.push(user?);
        }
        println!("{:#?}", users);
        Ok(users)
    }

    /// 查找指定用户，返回user
    pub fn find(user: &str) -> Result<User, Box<dyn std::error::Error>> {
        let conn = get_conn()?;
        // println!("{:#?}", user);
        let mut stmt = conn.prepare("SELECT id, name, password, level FROM user WHERE name = ?")?;

        let user = stmt.query_row([user], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                password: row.get(2)?,
                level: row.get(3)?,
            })
        })?;

        Ok(user)
    }
}
