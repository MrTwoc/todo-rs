use crate::{about, cmd::*};
use owo_colors::OwoColorize;

/*
    负责指令分发、处理第一级指令
    第一级指令：add、list、edit、del、sysinfo、clear、exit、quit、q、help
*/

pub fn command_handle(input: &str) {
    let args: Vec<&str> = input.trim().split_whitespace().collect();

    if args.is_empty() {
        return;
    }

    match args[0] {
        "add" => println!("add命令"),
        "list" => println!("list命令"),
        "edit" => println!("edit命令"),
        "del" => println!("del命令"),
        "sysinfo" => {
            command_sysinfo();
        }
        "clear" => {
            command_clear();
            // 清空控制台后重新打印标题
            print!("{}\n", &about::PRINT_TITLE.green());
            print!("{}\n", &about::TITLE_INFO);
        }
        _ => eprintln!("{} > {}", "未知命令".red(), args[0]),
    }
}
