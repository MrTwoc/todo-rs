use std::error::Error;

use chrono::NaiveDate;
// use comfy_table::{ColumnConstraint, ContentArrangement, Table, Width};

use crate::{
    cmd::{show_table, validate_and_parse_date},
    storage::save_json::TaskStorage,
    task_mod::{Target, TaskStatus},
};
use rayon::prelude::*;

// 将原来的UTF8_FULL中的双横线改为单横线,以下是样例
// pub const UTF8_FULL_F: &str = "││──╞─┼╡┆╌┼├┤┬┴┌┐└┘";
/*
┌────────┬──────────┬───────────────────────────────────┬────────────┬───────────┬──────┬──────┐
│ 任务ID ┆ 任务名称 ┆ 任务描述                          ┆ 截至日期   ┆ 状态      ┆ 分组 ┆ 级别 │
╞────────┼──────────┼───────────────────────────────────┼────────────┼───────────┼──────┼──────╡
│ 4      ┆ test     ┆ 无                                ┆ 1999-01-04 ┆ 🗓️ 进行中 ┆ 无   ┆ 正常 │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┤
│ 5      ┆ test     ┆ 无                                ┆ 1999-01-05 ┆ 🗓️ 进行中 ┆ 无   ┆ 正常 │
└────────┴──────────┴───────────────────────────────────┴────────────┴───────────┴──────┴──────┘
*/

impl Target {
    pub fn add(
        target_name: String,
        deadline: NaiveDate,
        description: Option<String>,
        group: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        let mut tasks = TaskStorage::read()?;

        // 创建任务对象
        tasks.push(Target {
            id: Some(tasks.iter().filter_map(|t| t.id).max().unwrap_or(0) + 1),
            task_name: target_name,
            deadline,
            task_status: TaskStatus::default(),
            description,
            group,
            // level: TaskLevel::Normal,
            task_value: 0,
        });

        TaskStorage::save(&tasks)?;
        if let Some(task) = tasks.last() {
            println!("添加成功=>\n任务：{:?}", task.task_name);
        }

        Ok(())
    }

    pub fn get_id() -> Result<u32, Box<dyn Error>> {
        let tasks = TaskStorage::read()?;
        let max_id = tasks.iter().filter_map(|t| t.id).max().unwrap_or(0);
        Ok(max_id + 1)
    }

    /// 批量删除有个bug，删除有真实数据的任务id，
    /// 后面跟一个不存在的id也会执行成功，但不会报错，
    /// 不过也会把真实存在的id删除
    pub fn del_many(ids: &[u32]) -> Result<(), Box<dyn Error>> {
        let mut tasks = TaskStorage::read()?;
        let initial_len = tasks.len();
        // tasks.retain(|task| task.id != Some(id));
        tasks.retain(|t| !ids.contains(&t.id.unwrap()));

        if tasks.len() < initial_len {
            TaskStorage::save(&tasks)?;
            println!("已删除任务: {:?}", ids);
        } else {
            eprintln!("找不到对应的任务: {:?}", ids);
        }
        Ok(())
    }

    pub fn list() -> Result<(), Box<dyn Error>> {
        let tasks = TaskStorage::read()?;
        show_table(&tasks)?;
        Ok(())
    }

    pub fn edit(args: &[&str]) -> Result<(), Box<dyn Error>> {
        let id: u32 = args[1]
            .parse()
            .map_err(|_| format!("无效的任务ID: {}", args[1]))?;

        let mut tasks = TaskStorage::read()?;

        let task_index = tasks
            .iter()
            .position(|t| t.id == Some(id))
            .ok_or(format!("未找到ID为{}的任务", id))?;

        // 获取可变任务引用
        let task = &mut tasks[task_index];

        // 解析并应用字段更新（从索引1开始，每两个参数为一组）
        for i in (2..args.len()).step_by(2) {
            let field = args[i];
            let value = args[i + 1];

            // 任务状态和任务级别用指令单独修改
            match field {
                "name" => task.task_name = value.to_string(),
                "deadline" => task.deadline = validate_and_parse_date(value)?,
                "description" => task.description = Some(value.to_string()),
                "group" => task.group = Some(value.to_string()),
                "value" => {
                    task.task_value = value.parse().unwrap_or(0);
                }
                _ => return Err(format!("不支持的字段: {}", field).into()),
            }
        }

        TaskStorage::save(&tasks)?;
        println!("成功修改");
        Ok(())
    }

    pub fn update_status(ids: &[u32], status: TaskStatus) -> Result<(), Box<dyn Error>> {
        let mut tasks = TaskStorage::read()?;
        tasks.par_iter_mut().for_each(|task| {
            if ids.contains(&task.id.unwrap_or(0)) {
                task.task_status = status.clone();
            }
        });
        TaskStorage::save(&tasks)?;
        println!("成功修改");

        Ok(())
    }

    pub fn find(keyword: &str) -> Result<(), Box<dyn Error>> {
        let tasks = TaskStorage::read()?;
        let keyword_lower = keyword.to_lowercase();

        // 使用闭包封装匹配逻辑
        let contains_keyword = |s: &str| s.to_lowercase().contains(&keyword_lower);

        let filtered_tasks = tasks
            .par_iter()
            .filter(|t| {
                contains_keyword(&t.task_name) // 匹配任务名称
                || t.description.as_deref().is_some_and(contains_keyword)
                || t.group.as_deref().is_some_and(contains_keyword)
            })
            .cloned()
            .collect::<Vec<_>>();

        if filtered_tasks.is_empty() {
            println!("未找到包含'{}'的任务", keyword);
            return Ok(());
        }

        // 调用表格函数，打印任务
        show_table(&filtered_tasks)?;
        Ok(())
    }
}
