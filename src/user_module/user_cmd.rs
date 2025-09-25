use owo_colors::OwoColorize;
use tracing::error;

use crate::user_module::{about_user::_USER_MODULE_INFO, user::User, user_mod::get_online_users};

pub fn command_user(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() == 1 {
        eprintln!("{}", "user 命令参数不足".red());
        return Ok(());
    }
    match args[1] {
        "list" => {
            User::list()?;
        }
        "add" => {
            add_user(args)?;
        }
        "del" => {
            del_user(args)?;
        }
        "level" => {
            set_level(args)?;
        }
        "find" => {
            find_user(args)?;
        }
        "pwd" => {
            set_pwd(args)?;
        }
        "online" => {
            online_users()?;
        }
        "help" => {
            user_help()?;
        }
        _ => {
            eprintln!("{} > {}", "未知命令".red(), args[1]);
            error!("未知命令: {:?}", args[1]);
        }
    }
    Ok(())
}

pub fn add_user(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        eprintln!("{}", "user add 命令参数不足".red());
        return Ok(());
    }
    let user = User {
        id: 0,
        name: args[2].to_string(),
        password: args[3].to_string(),
        level: 0,
    };
    User::add(&user)?;
    println!("用户添加成功 > {}", &args[2].green());
    Ok(())
}

pub fn del_user(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        eprintln!("{}", "user del 命令参数不足".red());
        return Ok(());
    }
    let user = User {
        // 这里也可以根据id/uuid进行删除
        id: args[2].parse::<u8>()?,
        name: "".to_string(),
        password: "".to_string(),
        level: 0,
    };
    User::del(&user)?;
    println!("用户删除成功 > {}", &args[2].green());
    Ok(())
}

pub fn set_level(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        eprintln!("{}", "user set_level 命令参数不足".red());
        return Ok(());
    }
    let user = User {
        id: args[2].parse::<u8>()?,
        name: "".to_string(),
        password: "".to_string(),
        level: args[3].parse::<u8>()?,
    };
    User::set_level(&user)?;
    println!("用户等级设置成功 > {}", &args[3].green());
    Ok(())
}

pub fn find_user(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        eprintln!("{}", "user find 命令参数不足".red());
        return Ok(());
    }
    if let Ok(user) = User::find(args[2]) {
        println!("{:#?}", user);
    } else {
        eprintln!("{}", "未找到用户".red());
    }
    Ok(())
}

/// 改变用户密码
/// user pwd old_pwd new_pwd
pub fn set_pwd(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        eprintln!("{}", "user pwd 命令参数不足".red());
        return Ok(());
    }
    let user = User {
        id: 0,
        name: args[2].to_string(),
        password: args[3].to_string(),
        level: 0,
    };
    println!("用户 {} 密码修改为 {}", &args[2].green(), &args[3].green());
    User::set_pwd(&user)?;
    println!("用户密码修改成功 > {}", &args[3].green());
    Ok(())
}

/// 显示当前在线用户
pub fn online_users() -> Result<(), Box<dyn std::error::Error>> {
    let guard = get_online_users().read().unwrap();
    println!("当前在线用户（{}位）：", guard.user_info.len());
    guard.user_info.iter().for_each(|(id, info)| {
        println!(
            "ID: {:<4} 用户名: {:<10} 权限等级: {}",
            id, info.username, info.user_level
        );
    });

    Ok(())
}

pub fn user_help() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", _USER_MODULE_INFO);
    Ok(())
}
