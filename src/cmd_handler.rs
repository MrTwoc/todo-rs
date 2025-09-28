use crate::{cmd::*, config::load_config::save_config, help, user_module::user_cmd::command_user};
use owo_colors::OwoColorize;
use std::time::Instant;
use tracing::{error, info};

/*
    负责指令分发、处理第一级指令
    第一级指令：add、list、edit、del、sysinfo、clear、exit、quit、q、help
*/

pub fn command_handle(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<&str> = input.split_whitespace().collect();

    if args.is_empty() {
        return Ok(());
    }

    let start = Instant::now();

    match args[0] {
        "add" => command_add(&args)?,
        "list" => command_list()?,
        "edit" => command_edit(&args)?,
        "del" => command_del(&args)?,
        "find" => command_find(&args)?,
        "status" => {
            command_update_status(&args)?;
        }
        "sysinfo" => {
            command_sysinfo()?;
        }
        "clear" => {
            command_clear();
            // 清空控制台后重新打印标题
            println!("{}", &help::PRINT_TITLE.green());
            println!("{}", &help::TITLE_INFO);
        }
        // 暂时这样
        "task" => {
            if args.len() == 1 {
                command_task_help();
            } else if args[1] == "help" {
                command_task_help();
            }
        }
        "user" => {
            command_user(&args)?;
        }
        "config" => {
            save_config(&args)?;
        }
        _ => {
            eprintln!("{} > {}", "未知命令".red(), args[0]);
            error!("未知命令: {:?}", args[0]);
        }
    }
    let duration = start.elapsed();
    println!("耗时: {}µs", &duration.as_micros());
    info!("耗时: {}µs", &duration.as_micros());

    Ok(())
}
