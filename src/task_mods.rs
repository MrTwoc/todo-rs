#[derive(Debug, Clone)]

pub struct Target {
    /// 任务id
    pub id: u32,
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

#[derive(Debug, Clone)]
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
        TargetStatus::Pending
    }
}
