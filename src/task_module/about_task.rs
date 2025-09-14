pub const _TASK_MODULE_INFO: &str = r#"
关于任务模块
=======================================
add <名称> <截止日期>
del <id> <id> <...> 【批量删除】
edit <id> <字段> <新值>
例如: edit 1 name 任务1号
包含字段: name, deadline, description, group, value
value(任务价值): 0~255
包含字段: name, deadline, description, group, level
list 列出所有任务
status <id> <id> <...> <状态> 【批量修改】
状态: 
pause(暂停), active(进行中), done(已完成),
cancel(已取消), outtime(已过期)
"#;
