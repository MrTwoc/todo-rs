use std::{
    error::Error,
    fs::{self},
    io,
};

use comfy_table::{ContentArrangement, Table, modifiers::UTF8_ROUND_CORNERS};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Target {
    /// 任务id
    pub id: Option<u32>,
    /// 任务名称
    pub target_name: String,
    /// 截止日期
    pub deadline: String,
    /// 任务状态
    pub target_status: TargetStatus,
    /// 任务描述
    pub description: Option<String>,
    /// 任务分组
    pub group: Option<String>,
    /// 任务级别
    pub level: TaskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskLevel {
    /// 一般
    Normal,
    /// 中等
    Medium,
    /// 重要
    High,
    // 秘密|机密|绝密
}

impl std::str::FromStr for TaskLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Normal" | "一般" => Ok(TaskLevel::Normal),
            "Medium" | "中等" => Ok(TaskLevel::Medium),
            "High" | "重要" => Ok(TaskLevel::High),
            _ => Err(format!("无效的任务级别: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetStatus {
    /// 等待开始
    Pending,
    /// 进行中
    InProgress,
    /// 已完成
    Completed,
    /// 已取消
    Canceled,
    /// 已过期
    Expired,
}

impl std::fmt::Display for TargetStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TargetStatus::Pending => "等待开始",
            TargetStatus::InProgress => "进行中",
            TargetStatus::Completed => "已完成",
            TargetStatus::Canceled => "已取消",
            TargetStatus::Expired => "已过期",
        };
        write!(f, "{s}")
    }
}

impl std::fmt::Display for TaskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TaskLevel::Normal => "一般",
            TaskLevel::Medium => "中等",
            TaskLevel::High => "重要",
        };
        write!(f, "{s}")
    }
}

impl Default for TargetStatus {
    fn default() -> Self {
        TargetStatus::InProgress
    }
}

impl Target {
    pub fn new() -> Self {
        Target {
            id: None,
            target_name: String::new(),
            deadline: String::new(),
            target_status: TargetStatus::default(),
            description: None,
            group: None,
            level: TaskLevel::Normal,
        }
    }
    pub fn add(
        target_name: String,
        deadline: String,
        description: Option<String>,
        group: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        let mut tasks = read_form_json()?;

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

        write_to_json(&tasks)?;
        tasks
            .last()
            .map(|task| println!("添加成功=>\n任务：{:?}", task.target_name));

        Ok(())
    }

    pub fn get_id() -> Result<u32, Box<dyn Error>> {
        let tasks = read_form_json()?;
        let max_id = tasks.iter().filter_map(|t| t.id).max().unwrap_or(0);
        Ok(max_id + 1)
    }

    pub fn del(id: u32) -> Result<(), Box<dyn Error>> {
        let mut tasks = read_form_json()?;
        let initial_len = tasks.len();
        tasks.retain(|task| task.id != Some(id));

        if tasks.len() < initial_len {
            write_to_json(&tasks)?;
            println!("已删除任务")
        } else {
            eprintln!("找不到对应的任务")
        }
        Ok(())
    }

    pub fn list() -> Result<(), Box<dyn Error>> {
        let mut table = Table::new();
        table
            .load_preset(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic);

        table.set_header(vec![
            "任务ID[ 🟢 ✅ ]",
            "任务名称",
            "任务描述",
            "截至日期",
            "状态",
            "分组",
            "级别",
        ]);

        for task in read_form_json()? {
            table.add_row(vec![
                task.id.map_or(0.to_string(), |v| v.to_string()),
                task.target_name,
                task.description.as_deref().map_or("无", |s| s).to_string(),
                task.deadline,
                task.target_status.to_string(),
                task.group.as_deref().map_or("无", |s| s).to_string(),
                task.level.to_string(),
            ]);
        }

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
        // let id = args.get(1).ok_or("缺少任务ID")?.parse::<u32>()?;
        let id: u32 = args[1]
            .parse()
            .map_err(|_| format!("无效的任务ID: {}", args[1]))?;

        let mut tasks = read_form_json()?;

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
                "deadline" => task.deadline = value.to_string(),
                "description" => task.description = Some(value.to_string()),
                "group" => task.group = Some(value.to_string()),
                "level" => {
                    task.level = match value.to_lowercase().as_str() {
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
        write_to_json(&tasks)?;
        println!("成功修改");
        Ok(())
    }
}

pub fn write_to_json(task: &[Target]) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create("task.json")?;
    let writer = io::BufWriter::new(file);
    serde_json::to_writer(writer, task)?;
    Ok(())
}

// 从json文件中读取电影列表
pub fn read_form_json() -> Result<Vec<Target>, Box<dyn Error>> {
    match fs::File::open("task.json") {
        Ok(f) => {
            let reader = io::BufReader::new(f);
            match serde_json::from_reader(reader) {
                Ok(task) => Ok(task),
                Err(e) if e.is_eof() => Ok(Vec::new()),
                Err(e) => {
                    println!("读取文件失败: {}", e);
                    Err(e.into())
                }
            }
        }
        // 优化: 文件不存在时不打印错误,直接返回空列表
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => {
            println!("读取文件失败: {}", e);
            Err(e.into())
        }
    }
}
