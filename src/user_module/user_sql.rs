use rusqlite::{ToSql, params};
use tracing::info;

use crate::storage::save_sqlite::get_conn;
use crate::user_module::user::User;

impl User {
    pub fn user_db_init() -> Result<(), Box<dyn std::error::Error>> {
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
         INSERT INTO user (name, password, level) VALUES ('admin', 'admin', 1);
         INSERT INTO user (name, password, level) VALUES ('user', 'user', 0);
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
    /// 支持按用户名/用户id/用户等级查找: &str和u8类型
    pub fn find<T>(user: T) -> Result<User, Box<dyn std::error::Error>>
    where
        T: ToSql + Clone,
    {
        let conn = get_conn()?;
        // println!("{:#?}", user);
        let mut stmt = conn.prepare(
            "SELECT id, name, password, level FROM user WHERE name = ? OR id = ? OR level = ?",
        )?;

        let user = stmt.query_row(params![user, user, user], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                password: row.get(2)?,
                level: row.get(3)?,
            })
        })?;
        info!("Find user > {}", &user.name);
        Ok(user)
    }

    pub fn add(user: &User) -> Result<(), Box<dyn std::error::Error>> {
        let conn = get_conn()?;
        let mut stmt = conn.prepare("INSERT INTO user (name, password) VALUES (?, ?)")?;
        stmt.execute((&user.name, &user.password))?;
        info!("Add user > {}", &user.name);
        Ok(())
    }

    pub fn del(user: &User) -> Result<(), Box<dyn std::error::Error>> {
        let conn = get_conn()?;
        let mut stmt = conn.prepare("DELETE FROM user WHERE id = ?")?;
        stmt.execute((&user.id,))?;
        info!("Del user > {}", &user.id);
        Ok(())
    }

    pub fn set_level(user: &User) -> Result<(), Box<dyn std::error::Error>> {
        let conn = get_conn()?;
        let mut stmt = conn.prepare("UPDATE user SET level = ? WHERE id = ?")?;
        stmt.execute((&user.level, &user.id))?;
        info!("Set user level > {} to {}", &user.id, &user.level);
        Ok(())
    }

    /// 修改用户密码
    pub fn set_pwd(user: &User) -> Result<(), Box<dyn std::error::Error>> {
        let conn = get_conn()?;
        let mut stmt = conn.prepare("UPDATE user SET password = ? WHERE id = ?")?;
        stmt.execute((&user.password, &user.id))?;
        info!("Set user pwd > {} to {}", &user.id, &user.password);
        Ok(())
    }
}
