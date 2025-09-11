pub const _USER_MODULE_INFO: &str = r#"
关于用户模块
User Struct：
struct User {
    id: u8,
    // uuid 估计添加功能
    name: String,
    password: String,
    // 权限等级
    level: u8,
}
用户模块包含指令：
user add <用户名> <密码>
user del <用户id>
user edit <用户id> <字段> <新值>
user list
user info <用户id>
待实现：
user find <关键词：用户名/用户id/用户等级>
"#;
