use tracing::info;

use crate::{cmd::validate_and_parse_date, storage::save_sqlite::get_conn, task_mod::Target};

/*
    任务表结构
    id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_name TEXT UNIQUE,
            deadline TEXT,
            task_status TEXT DEFAULT 'Active',
            description TEXT DEFAULT '无',
            'task_group' TEXT DEFAULT '无',
            task_value INTEGER DEFAULT 0
            todo:
            is_del：是否被删除，默认0为未删除，1为已删除
            another: 任务创建者，已登录就记录创建者，没登陆就'无'
*/

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
        info!("[sqlite]任务表创建成功 [1/2]");
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM tasks", (), |row| row.get(0))?;

        if count == 0 {
            conn.execute_batch(
                "INSERT OR IGNORE INTO tasks (task_name, deadline) 
     VALUES 
        ('任务1', '2026-01-01'),
        ('任务2', '2026-01-02');",
            )?;
        }
        info!("[sqlite]测试任务插入成功 [2/2]");
        info!("[sqlite]任务初始化完成");
        Ok(())
    }

    // 向数据库中添加任务
    pub fn sql_add(task: &Target) -> Result<(), Box<dyn std::error::Error>> {
        // 检查任务名称是否重复
        if Self::find_by_name(&task.task_name) {
            return Err("任务名称重复".into());
        }
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
        info!("[sqlite]任务 {} 插入成功", &task.task_name);
        Ok(())
    }

    // 从数据库中删除任务
    pub fn sql_del(task_id: &u32) -> Result<(), Box<dyn std::error::Error>> {
        // println!("[sqlite]sql_del执行");
        if !Self::find_by_id(task_id) {
            return Err("任务不存在".into());
        }
        let conn = get_conn()?;
        conn.execute("DELETE FROM tasks WHERE id = ?", (task_id,))?;
        info!("[sqlite]任务 {} 被删除", task_id);

        Ok(())
    }

    // 获取数据库所有任务
    pub fn get_all_tasks() -> Result<Vec<Target>, Box<dyn std::error::Error>> {
        // println!("[sqlite]get_all_tasks执行");
        let conn = get_conn()?;
        let mut stmt = conn
            .prepare("SELECT id, task_name, deadline, task_status, description, task_group, task_value FROM tasks")
            .unwrap();
        let rows = stmt.query_map([], |row| {
            Ok(Target {
                id: Some(row.get(0)?),
                task_name: row.get(1)?,
                deadline: row.get(2)?,
                task_status: row.get(3)?,
                description: row.get(4)?,
                group: row.get(5)?,
                task_value: row.get(6)?,
            })
        })?;
        // 使用collect来简化处理
        let tasks: Result<Vec<Target>, rusqlite::Error> = rows.collect();
        Ok(tasks?)
    }

    // 根据id查找任务,如果存在返回true,否则返回false
    pub fn find_by_id(task_id: &u32) -> bool {
        // println!("[sqlite]find_by_id执行");
        let conn = get_conn().unwrap();
        let mut stmt = conn.prepare("SELECT id FROM tasks WHERE id = ?").unwrap();
        let rows = stmt.query_map([*task_id], |row| row.get(0)).unwrap();
        for row in rows {
            let id: u32 = row.unwrap();
            if id == *task_id {
                return true;
            }
        }
        false
    }

    // 根据任务名称查找是否有重复
    fn find_by_name(task_name: &str) -> bool {
        // println!("[sqlite]find_by_name执行");
        let conn = get_conn().unwrap();
        let mut stmt = conn
            .prepare("SELECT task_name FROM tasks WHERE task_name = ?")
            .unwrap();
        let rows = stmt.query_map([task_name], |row| row.get(0)).unwrap();
        for row in rows {
            let name: String = row.unwrap();
            if name == *task_name {
                return true;
            }
        }
        false
    }

    // 根据ID查询任务
    // pub fn find_task(task_id: &u32) -> Result<Target, Box<dyn std::error::Error>> {
    //     let conn = get_conn()?;
    //     // name, deadline, description, group, value
    //     let mut stmt = conn.prepare("SELECT task_name, deadline, description, task_group, task_value FROM tasks WHERE id = ?").unwrap();
    //     let target = stmt.query_row([*task_id], |row| {
    //         Ok(Target {
    //             id: Some(*task_id),
    //             task_name: row.get(0)?,
    //             deadline: row.get(1)?,
    //             task_status: TaskStatus::default(),
    //             description: row.get(2)?,
    //             group: row.get(3)?,
    //             task_value: row.get(4)?,
    //         })
    //     })?;

    //     Ok(target)
    // }

    /*
       更新任务信息
       可以按参数长度适配，参数为空时不更新
    */
    pub fn sql_edit(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        // println!("[sqlite]sql_edit执行");

        // println!("sql_edit参数: {:?}", args);
        // 判断任务id是否存在
        let task_id = args[1].parse::<u32>().unwrap_or(0);
        if !Self::find_by_id(&task_id) {
            return Err("任务ID不存在".into());
        }
        // 从数据库中查询旧任务信息，然后可以直接update。
        // 使用for循环遍历args，根据参数长度适配，参数为空时不更新
        // 最多5个参数，分别是任务名称、截止日期、任务描述、任务分组、任务价值，所以最多循环5次
        // let old_task = Self::find_task(&task_id)?;'
        let conn = get_conn()?;
        for i in (2..args.len()).step_by(2) {
            let field = args[i];
            let value = args[i + 1];

            println!("sql_edit字段: {:?}", field);
            println!("sql_edit值: {:?}", value);
            match field {
                "name" => {
                    // 检查任务名称是否重复
                    if Self::find_by_name(value) {
                        return Err("任务名称重复".into());
                    }
                    // 更新任务名称
                    conn.execute(
                        "UPDATE tasks SET task_name = ? WHERE id = ?",
                        (value, task_id),
                    )?;
                }
                "deadline" => {
                    // 检查截止日期是否有效
                    if validate_and_parse_date(value).is_err() {
                        return Err("无效的截止日期格式".into());
                    }
                    // 更新任务截止日期
                    conn.execute(
                        "UPDATE tasks SET deadline = ? WHERE id = ?",
                        (value, task_id),
                    )?;
                }
                "description" => {
                    // 更新任务描述
                    conn.execute(
                        "UPDATE tasks SET description = ? WHERE id = ?",
                        (value, task_id),
                    )?;
                }
                "group" => {
                    // 更新任务分组
                    conn.execute(
                        "UPDATE tasks SET task_group = ? WHERE id = ?",
                        (value, task_id),
                    )?;
                }
                "value" => {
                    // 检查任务价值是否有效
                    if value.parse::<u8>().is_err() {
                        return Err("无效的任务价值格式".into());
                    }
                    // 更新任务价值
                    conn.execute(
                        "UPDATE tasks SET task_value = ? WHERE id = ?",
                        (value.parse::<u8>().unwrap(), task_id),
                    )?;
                }
                _ => return Err(format!("不支持的字段: {}", field).into()),
            }
        }

        Ok(())
    }
}
