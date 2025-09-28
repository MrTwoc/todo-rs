use std::process::Command;
use std::result::Result;

use crate::{task_mod::*, task_module};
use chrono::NaiveDate;
// use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
// use comfy_table::{ColumnConstraint, ContentArrangement, Table, Width};
// use owo_colors::*;
use std::error::Error;
use textwrap::wrap;
use tracing::info;

use owo_colors::OwoColorize;
use unicode_width::UnicodeWidthStr;

/*
    负责处理指令
*/

pub fn command_sysinfo() -> Result<(), Box<dyn Error>> {
    let mut sys = sysinfo::System::new();
    sys.refresh_all();

    if let Some(process) = sys.process(sysinfo::Pid::from(std::process::id() as usize)) {
        println!(
            "内存使用 > {} MB\nCPU使用率 > {:.1}%",
            &process.memory() / 1024 / 1024,
            &process.cpu_usage()
        );
    }
    Ok(())
}

pub fn command_clear() {
    // 在Windows上执行cls命令清空控制台
    if let Err(e) = Command::new("cmd").arg("/c").arg("cls").status() {
        eprintln!("清空控制台失败: {}", e);
    }
    info!("clear");
}

pub fn validate_and_parse_date(date_str: &str) -> Result<NaiveDate, Box<dyn Error>> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| {
        format!(
            "无效日期格式: {}，请使用YYYY-MM-DD格式(例如: 1999-1-1或1999-01-01)",
            date_str
        )
        .into()
    })
}

pub fn command_add(args: &[&str]) -> Result<(), Box<dyn Error>> {
    // 判断args是否为空
    if args.len() < 3 {
        return Err(
            "参数不足,使用方法: add <任务名称> <截止时间> option[描述] option[分组]".into(),
        );
    }
    let deadline = validate_and_parse_date(args[2])?;

    // 仅负责参数解析和类型转换
    Target::add(
        args[1].to_string(),
        // args[2].to_string(),
        deadline,
        args.get(3).map(|s| s.to_string()),
        args.get(4).map(|s| s.to_string()),
    )?;
    info!("{:?}", &args);

    Ok(())
}
pub fn command_list() -> Result<(), Box<dyn Error>> {
    Target::list()?;
    info!("list");

    Ok(())
}

/*
实现方式：
接收用户输入的ID并查找，如果存在修改，如果不存在则提醒用户
不强制要求参数数量，可以根据关键词匹配用户要修改哪些字段，但最少要输入一个字段，少于两个参数则提醒用户
参数格式：<任务ID> <字段> <新值>
字段：name、deadline、description、group
例如：
edit <任务ID> name '新任务名称'
edit <任务ID> deadline '新截至时间'
edit <任务ID> name '新任务名称' deadline '新截至时间'
edit <任务ID> name '新任务名称' deadline '新截至时间' description '新任务描述' group '新分组'
*/
pub fn command_edit(args: &[&str]) -> Result<(), Box<dyn Error>> {
    // 0号参数为指令，1号参数为任务ID，2号参数为字段，3号参数为新值，4号参数为字段，5号参数为新值，以此类推
    // 检查参数数量是否正确，且配对
    if args.len() < 3 || (args.len() - 1) % 2 == 0 {
        eprintln!(
            "执行失败: 指令参数错误
请输入: edit <任务ID> <字段> [修改内容]...
例如: edit 1 name 任务1号
包含字段: name, deadline, description, group, value
value(任务价值): 0~255
        "
        );
        return Ok(());
    }
    Target::edit(args)?;
    info!("{:?}", &args);

    Ok(())
}
pub fn command_del(args: &[&str]) -> Result<(), Box<dyn Error>> {
    // 判断args是否为空
    if args.len() < 2 {
        return Err("请输入要删除的任务ID，多个ID用空格分隔".into());
    }
    // let id = args.get(1).ok_or("缺少任务ID")?.parse::<u32>()?;
    let ids: Vec<u32> = args[1..]
        .iter()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    Target::del_many(&ids)?;
    info!("{:?}", &args);

    Ok(())
}

pub fn command_update_status(args: &[&str]) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        return Err("参数不匹配，使用方法: status <任务状态> <状态ID>\n状态分类：pause, active, done, cancel, outtime".into());
    }

    // let status = match args[1] {
    //     "pause" => TaskStatus::Pause,
    //     "active" => TaskStatus::Active,
    //     "done" => TaskStatus::Done,
    //     "cancel" => TaskStatus::Cancel,
    //     "outtime" => TaskStatus::OutTime,
    //     _ => return Err("无效的状态参数，可选值: pause, active, done, cancel, outtime".into()),
    // };
    // let ids: Vec<u32> = args[2..]
    //     .iter()
    //     .map(|s| s.parse())
    //     .collect::<Result<_, _>>()?;

    // Target::update_status(&ids, status)?;
    Target::sql_update_status(args)?;
    info!("{:?}", &args);

    Ok(())
}

/// 根据关键词，查找包含关键字的所有任务
/// 关键词可以是任务名称、任务描述、任务分组
pub fn command_find(args: &[&str]) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        return Err("请输入要查找的关键词".into());
    }
    let keyword = args[1];

    Target::find(keyword)?;
    info!("{:?}", &args);

    Ok(())
}

pub fn command_task_help() {
    println!("{}", &task_module::about_task::_TASK_MODULE_INFO);
}

const REGEX_COLOR: &str = r"\x1B\[([0-9]{1,3}(;[0-9]{1,3})*)?[mGK]";
/// 计算带颜色文本的显示宽度（忽略ANSI转义序列）
fn colored_text_width(text: &str) -> usize {
    // 移除ANSI颜色转义序列
    let re = Regex::new(REGEX_COLOR).unwrap();
    let cleaned = re.replace_all(text, "");
    cleaned.width()
}

/// 带颜色的文本左对齐
fn colored_left_pad(text: &str, width: usize) -> String {
    let current_width = colored_text_width(text);
    if current_width < width {
        format!("{}{}", text, " ".repeat(width - current_width))
    } else {
        text.to_string()
    }
}

pub fn show_table(tasks: &[Target]) -> Result<(), Box<dyn Error>> {
    // 表头渲染
    println!(
        " {} {} {} {} {} {} {}",
        colored_left_pad(
            &colored_left_pad("[ID]", 3)
                // .on_blue()
                // .white()
                .bright_green()
                .bold()
                .to_string(),
            3
        ),
        colored_left_pad(
            &colored_left_pad("[Target]", 15)
                // .on_blue()
                // .white()
                .bright_green()
                .bold()
                .to_string(),
            15
        ),
        colored_left_pad(
            &colored_left_pad("[Description]", 30)
                // .on_cyan()
                // .white()
                .bright_green()
                .bold()
                .to_string(),
            30
        ),
        colored_left_pad(
            &colored_left_pad("[Deadline]", 10)
                // .on_bright_red()
                // .white()
                .bright_green()
                .bold()
                .to_string(),
            10
        ),
        colored_left_pad(
            &colored_left_pad("[Status]", 10)
                // .on_green()
                // .white()
                .bright_green()
                .bold()
                .to_string(),
            10
        ),
        colored_left_pad(
            &colored_left_pad("[Group]", 10)
                // .on_magenta()
                // .white()
                .bright_green()
                .bold()
                .to_string(),
            10
        ),
        colored_left_pad(
            &colored_left_pad("[Value]", 10)
                // .on_red()
                // .white()
                .bright_green()
                .bold()
                .to_string(),
            10
        )
    );
    println!("{}", "-".repeat(100));

    tasks.iter().for_each(|task| {
        let desc_str = task.description.as_deref().unwrap_or("无");
        let wrapped_desc = wrap(desc_str, 30);
        let id_str = task.id.unwrap().to_string();
        let target_name = task.task_name.to_string();
        let deadline_str = task.deadline.format("%Y-%m-%d").to_string();
        let status_str = task.task_status.to_string();
        let group_str = task.group.as_deref().unwrap_or("无").to_string();
        let value_str = task.task_value.to_string();

        // 预计算首行固定内容
        let id_padded = colored_left_pad(&id_str, 3);
        let name_padded = colored_left_pad(&target_name, 15);
        let deadline_padded = colored_left_pad(&deadline_str, 10);
        let status_padded = colored_left_pad(&status_str, 10);
        let group_padded = colored_left_pad(&group_str, 10);
        let value_padded = colored_left_pad(&value_str, 10);

        // 处理首行
        if let Some(first_desc) = wrapped_desc.first() {
            println!(
                " {} {} {} {} {} {} {}",
                id_padded,
                name_padded,
                colored_left_pad(&first_desc.white().to_string(), 30),
                deadline_padded,
                status_padded,
                group_padded,
                value_padded
            );
        }

        // 处理剩余描述行
        for desc_line in wrapped_desc.iter().skip(1) {
            println!(
                " {} {} {} {} {} {} {}",
                "   ",             // ID列空白占位
                "               ", // 名称列空白占位
                colored_left_pad(&desc_line.to_string(), 30),
                "          ", // 日期列空白占位
                "          ", // 状态列空白占位
                "          ", // 分组列空白占位
                "          "  // 价值列空白占位
            );
        }
    });
    Ok(())
}
