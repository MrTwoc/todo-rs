use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Target {
    /// 任务id
    pub id: Option<u32>,
    /// 任务名称
    pub target_name: String,
    /// 截止日期
    pub deadline: chrono::NaiveDate,
    /// 任务状态
    pub target_status: TargetStatus,
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
pub enum TargetStatus {
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

impl std::fmt::Display for TargetStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TargetStatus::Pause => "⏸️ 暂停",
            TargetStatus::Active => "🟢 进行",
            TargetStatus::Done => "✅ 完成",
            TargetStatus::Cancel => "❌ 取消",
            TargetStatus::OutTime => "⏳ 过期",
        };
        write!(f, "{s}")
    }
}
