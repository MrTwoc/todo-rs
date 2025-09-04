use owo_colors::OwoColorize;
use tracing::{error, info};

use crate::user::user::User;

pub fn command_user(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() == 1 {
        eprintln!("{}", "user 命令参数不足".red());
        return Ok(());
    }
    match args[1] {
        "list" => {
            User::list()?;
            info!("args: {:?}", args);
        }
        _ => {
            eprintln!("{} > {}", "未知命令".red(), args[1]);
            error!("未知命令: {:?}", args[1]);
        }
    }
    Ok(())
}
