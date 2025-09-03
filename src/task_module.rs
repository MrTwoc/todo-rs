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
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum TaskLevel {
//     // 或者给任务级别，改成任务价值，例如价值100
//     /// 低
//     Low,
//     /// 一般
//     Normal,
//     /// 中等
//     Medium,
//     /// 重要
//     High,
//     // 把任务级别改为价值后，把[秘密|机密|绝密]变成隐藏类型
// }

// impl std::str::FromStr for TaskLevel {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "Normal" | "一般" => Ok(TaskLevel::Normal),
//             "Medium" | "中等" => Ok(TaskLevel::Medium),
//             "High" | "重要" => Ok(TaskLevel::High),
//             _ => Err(format!("无效的任务级别: {}", s)),
//         }
//     }
// }

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

// impl std::fmt::Display for TaskLevel {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let s = match self {
//             TaskLevel::Low => "低",
//             TaskLevel::Normal => "正常",
//             TaskLevel::Medium => "中",
//             TaskLevel::High => "高",
//         };
//         write!(f, "{s}")
//     }
// }
