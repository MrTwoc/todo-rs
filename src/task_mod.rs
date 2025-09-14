use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Target {
    /// 任务id
    pub id: Option<u32>,
    /// 任务名称
    pub task_name: String,
    /// 截止日期
    pub deadline: chrono::NaiveDate,
    /// 任务状态
    pub task_status: TaskStatus,
    /// 任务描述
    pub description: Option<String>,
    /// 任务分组
    pub group: Option<String>,
    /// 任务级别
    // pub level: TaskLevel,

    /// 任务价值,取代任务级别(0~255)
    pub task_value: u8,
    // 任务标签: 可以存储任务的一些额外信息,如创建者、创建时间、任务类型等
    // pub tags: Option<HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    /// 暂停中
    Pause,
    /// 进行中
    #[default]
    Active,
    /// 已完成
    Done,
    /// 已取消
    Cancel,
    /// 已过期
    OutTime,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TaskStatus::Pause => "⏸️ 暂停",
            TaskStatus::Active => "🟢 进行",
            TaskStatus::Done => "✅ 完成",
            TaskStatus::Cancel => "❌ 取消",
            TaskStatus::OutTime => "⏳ 过期",
        };
        write!(f, "{s}")
    }
}
