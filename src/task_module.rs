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
    pub level: TaskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskLevel {
    /// 低
    Low,
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum TargetStatus {
    /// 等待开始
    Pending,
    /// 进行中
    #[default]
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
            TaskLevel::Low => "低",
            TaskLevel::Normal => "正常",
            TaskLevel::Medium => "中",
            TaskLevel::High => "高",
        };
        write!(f, "{s}")
    }
}
