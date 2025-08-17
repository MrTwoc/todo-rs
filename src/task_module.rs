use std::{
    error::Error,
    fs::{self},
    io,
};

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
        }
    }
    pub fn add(&self) -> Result<(), Box<dyn Error>> {
        let mut tasks = read_form_json()?;
        tasks.push(self.clone());
        write_to_json(&tasks)?;
        println!(
            "添加成功=>
        任务：{:?}",
            &self.target_name
        );
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
        let tasks = read_form_json()?;

        for task in tasks {
            println!(
                "任务ID:{:?} 任务名称:{:?} 截至:{:?} 状态:{:?} 分组:{:?}",
                task.id, task.target_name, task.deadline, task.target_status, task.group
            )
        }
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

            // 任务状态单独修改
            match field {
                "name" => task.target_name = value.to_string(),
                "deadline" => task.deadline = value.to_string(),
                "description" => task.description = Some(value.to_string()),
                "group" => task.group = Some(value.to_string()),

                _ => eprintln!("不支持的字段: {}", field),
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
    let file = match fs::File::open("task.json") {
        Ok(f) => f,
        Err(e) => {
            println!("读取文件失败: {}", e);
            return Ok(Vec::new());
        }
    };
    let reader = io::BufReader::new(file);
    // let task: Vec<Target> = serde_json::from_reader(reader)?;
    match serde_json::from_reader(reader) {
        Ok(task) => Ok(task),
        // 若是空文件，则返回空列表，避免eof崩溃
        Err(e) if e.is_eof() => Ok(Vec::new()),
        Err(e) => Err(e.into()),
    }
}
