use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/*
    用户信息可以存在数据库中，数据库中存储用户信息，但不存储登录状态
*/
pub struct User {
    pub id: u8,
    pub name: String,
    pub password: String,
    /// 用户级别 0:普通用户 1:管理员
    /// 支持 0~255 级别细分
    /// 级别权重(可以写入config),用户级别比任务级别高几级，可以直接操作任务
    pub level: u8,
}
#[derive(Debug)]
pub struct LoginInfo {
    // 这里会有个bug，如果不活跃的账户的程序一直运行不关闭的话，
    // 登陆信息也不会清除，前期先不考虑
    pub is_login: bool,
}

pub struct OnineUser {
    /// u8:用户ID、这里可以将参数类型改为 DashMap
    pub user_info: Arc<RwLock<HashMap<u8, LoginInfo>>>,
}
