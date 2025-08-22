use std::error::Error;

use chrono::NaiveDate;
use comfy_table::{ColumnConstraint, ContentArrangement, Table, Width};

use crate::{
    cmd::validate_and_parse_date,
    storage::TaskStorage,
    task_module::{Target, TargetStatus, TaskLevel},
};

// 将原来的UTF8_FULL中的双横线改为单横线
pub const UTF8_FULL_F: &str = "││──╞─┼╡┆╌┼├┤┬┴┌┐└┘";

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
            target_name,
            deadline,
            target_status: TargetStatus::default(),
            description,
            group,
            level: TaskLevel::Normal,
        });

        TaskStorage::save(&tasks)?;
        // tasks
        //     .last()
        //     .map(|task| println!("添加成功=>\n任务：{:?}", task.target_name));
        if let Some(task) = tasks.last() {
            println!("添加成功=>\n任务：{:?}", task.target_name);
        }

        Ok(())
    }

    pub fn get_id() -> Result<u32, Box<dyn Error>> {
        let tasks = TaskStorage::read()?;
        let max_id = tasks.iter().filter_map(|t| t.id).max().unwrap_or(0);
        Ok(max_id + 1)
    }

    pub fn del(id: u32) -> Result<(), Box<dyn Error>> {
        let mut tasks = TaskStorage::read()?;
        let initial_len = tasks.len();
        tasks.retain(|task| task.id != Some(id));

        if tasks.len() < initial_len {
            TaskStorage::save(&tasks)?;
            println!("已删除任务")
        } else {
            eprintln!("找不到对应的任务")
        }
        Ok(())
    }

    pub fn list() -> Result<(), Box<dyn Error>> {
        let mut table = Table::new();
        table
            // .apply_modifier(UTF8_HORIZONTAL_ONLY)
            // // .load_preset(UTF8_ROUND_CORNERS)
            // .set_content_arrangement(ContentArrangement::Dynamic)
            // .load_preset(UTF8_FULL)
            .load_preset(UTF8_FULL_F)
            .set_content_arrangement(ContentArrangement::Dynamic);

        // [ 🟢 ✅ ]
        table.set_header(vec![
            "任务ID",
            "任务名称",
            "任务描述",
            "截至日期",
            "状态",
            "分组",
            "级别",
        ]);
        for task in TaskStorage::read()? {
            table.add_row(vec![
                task.id.map_or(0.to_string(), |v| v.to_string()),
                task.target_name,
                task.description.as_deref().map_or("无", |s| s).to_string(),
                task.deadline.format("%Y-%m-%d").to_string(),
                task.target_status.to_string(),
                task.group.as_deref().map_or("无", |s| s).to_string(),
                task.level.to_string(),
            ]);
        }
        // table
        //     .column_mut(0)
        //     .unwrap()
        //     .set_constraint(ColumnConstraint::Absolute(Width::Fixed(8)));

        table
            .column_mut(2)
            .unwrap()
            .set_constraint(ColumnConstraint::Absolute(Width::Fixed(35)));

        println!("{table}");
        Ok(())
    }

    pub fn find_by_id(_id: u32) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    pub fn find_by_name(_name: &str) -> Result<(), Box<dyn Error>> {
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
                "name" => task.target_name = value.to_string(),
                "deadline" => task.deadline = validate_and_parse_date(value)?,
                "description" => task.description = Some(value.to_string()),
                "group" => task.group = Some(value.to_string()),
                "level" => {
                    task.level = match value.to_lowercase().as_str() {
                        "low" => TaskLevel::Low,
                        "normal" => TaskLevel::Normal,
                        "medium" => TaskLevel::Medium,
                        "high" => TaskLevel::High,
                        _ => {
                            return Err(format!("不支持的任务级别: {}", value).into());
                        }
                    }
                }
                _ => return Err(format!("不支持的字段: {}", field).into()),
            }
        }

        TaskStorage::save(&tasks)?;
        println!("成功修改");
        Ok(())
    }
}
