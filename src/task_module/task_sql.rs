use tracing::info;

use crate::{storage::save_sqlite::get_conn, task_mod::Target};

impl Target {
    pub fn target_db_init() -> Result<(), Box<dyn std::error::Error>> {
        //   println!("target_init：已执行");
        let conn = get_conn()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_name TEXT UNIQUE,
            deadline TEXT,
            task_status TEXT DEFAULT 'Active',
            description TEXT DEFAULT '无',
            'task_group' TEXT DEFAULT '无',
            task_value INTEGER DEFAULT 0
            )",
            (),
        )?;
        info!("任务表创建成功 [1/2]");
        conn.execute_batch(
            "INSERT OR IGNORE INTO tasks (task_name, deadline) 
     VALUES 
        ('任务1', '2026-01-01'),
        ('任务2', '2026-01-02');",
        )?;
        info!("测试任务插入成功 [2/2]");
        info!("任务初始化完成");
        Ok(())
    }

    pub fn sql_add(task: &Target) -> Result<(), Box<dyn std::error::Error>> {
        println!("sql_add函数执行！");
        let conn = get_conn()?;
        conn.execute(
            "INSERT OR IGNORE INTO tasks (task_name, deadline, description, task_group) 
     VALUES 
        (?, ?, COALESCE(?, '无'), COALESCE(?, '无'))",
            (
                &task.task_name,
                &task.deadline,
                &task.description,
                &task.group,
            ),
        )?;
        info!("任务 {} 插入成功", &task.task_name);
        Ok(())
    }
}
