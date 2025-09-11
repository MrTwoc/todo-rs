use tracing::info;

use crate::{storage::save_sqlite::get_conn, task_module::Target};

impl Target {
    pub fn target_init() -> Result<(), Box<dyn std::error::Error>> {
        let conn = get_conn()?;
        conn.execute(
            "CREATE TABLE target(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            task_value INTEGER,
            status INTEGER DEFAULT 0
            )",
            (),
        )?;
        info!("数据库和任务表创建成功 [1/2]");
        Ok(())
    }
}
