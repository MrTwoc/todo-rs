pub const PRINT_TITLE: &str = r#"
___________        .___
\__    ___/___   __| _/____           _______  ______
  |    | /  _ \ / __ |/  _ \   ______ \_  __ \/  ___/
  |    |(  <_> ) /_/ (  <_> ) /_____/  |  | \/\___ \
  |____| \____/\____ |\____/           |__|  /____  >
                    \/                            \/
"#;

// pub const ABOUT_INFO: &str = r#"
// author: MrTwoc
// email: 1191422391@qq.com
// blog: https://mrtowc.xlog.app/
// github: https://github.com/MrTwoc
// "#;

pub const TITLE_INFO: &str = r#"
Todo-rs =>
'help'将显示帮助信息
'exit'/'quit'/'q'将退出程序
"#;

pub const HELP_INFO: &str = r#"
'help'将显示帮助信息
'exit'/'quit'/'q'将退出程序
'clear'清空控制台
'sysinfo'显示内存与cpu占用率
=======================================
'user help' 显示用户模块帮助信息
'task help' 显示任务模块帮助信息
"#;

/*
'add'/'del'/'edit'/'list'将显示相应信息

add <名称> <截止日期>
del <id> <id> <...> 【批量删除】
edit <id> <字段> <新值>
包含字段: name, deadline, description, group, level
list 列出所有任务
status <id> <id> <...> <状态> 【批量修改】
状态: pause(暂停), active(进行中), done(已完成), cancel(已取消), outtime(已过期)
*/

/*
相关的emoji推荐,用于美化输出
🔴
✅✔️🟢已经完成  🟡⏸️待定中 🔥已过期
❎取消任务
🔵 次要项目  ✅ 需要完成
⏳ 进行中的任务 ⚠️ 重要提醒
📅 计划任务 🔄 重复性任务 ❌ 取消的任务
⭐ 高优先级 🔍 需要复查 📌 长期任务
💡 新想法 ❗️ 常见错误 ❔ 常见问题

🚧 进行中任务 🎉 已完成庆祝 ⏸️ 暂停的任务
🗓️ 日程安排 📋 待处理事项 📈 进度追踪
🔔 提醒通知 📦 新任务入库 🛠️ 需要调整
🎯 目标达成
🔖 带标签任务
📎 附件关联
👥 协作任务
🔒 加密任务
🌐 网络相关
*/
