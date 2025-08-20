use std::process::Command;
use std::result::Result;

use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use crate::task_module::*;
use std::error::Error;

/*
    负责处理指令
*/

pub fn command_sysinfo() -> Result<(), Box<dyn Error>> {
    let mut sys = sysinfo::System::new();
    sys.refresh_all();

    if let Some(process) = sys.process(sysinfo::Pid::from(std::process::id() as usize)) {
        println!(
            "内存使用 > {} MB\nCPU使用率 > {:.1}%",
            process.memory() / 1024 / 1024,
            process.cpu_usage()
        );
    }
    Ok(())
}

pub fn command_clear() {
    // 在Windows上执行cls命令清空控制台
    if let Err(e) = Command::new("cmd").arg("/c").arg("cls").status() {
        eprintln!("清空控制台失败: {}", e);
    }
}

pub fn command_add(args: &[&str]) -> Result<(), Box<dyn Error>> {
    // 判断args是否为空
    if args.len() < 3 {
        return Err(
            "参数不足,使用方法: add <任务名称> <截止时间> option[描述] option[分组]".into(),
        );
    }

    // 仅负责参数解析和类型转换
    Target::add(
        args[1].to_string(),
        args[2].to_string(),
        args.get(3).map(|s| s.to_string()),
        args.get(4).map(|s| s.to_string()),
    )?;
    Ok(())
}
pub fn command_list() -> Result<(), Box<dyn Error>> {
    Target::list()?;
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
包含字段: name, deadline, description, group, level
level: low,normal, medium, high"
        );
        return Ok(());
    }
    Target::edit(&args)?;

    Ok(())
}
pub fn command_del(args: &[&str]) -> Result<(), Box<dyn Error>> {
    // 判断args是否为空，以及超过两个参数的话就无视掉
    if args.len() < 2 {
        eprintln!(
            "执行失败: 指令参数不足
请输入: del <任务ID>
例如: del 1"
        );
        return Ok(());
    }
    let id = args.get(1).ok_or("缺少任务ID")?.parse::<u32>()?;

    Target::del(id)?;
    Ok(())
}

pub fn table_demo() {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(80)
        .set_header(vec![
            Cell::new("Header1").add_attribute(Attribute::Bold),
            Cell::new("Header2").fg(Color::Green),
            Cell::new("Header3"),
        ])
        .add_row(vec![
             Cell::new("This is a bold text").add_attribute(Attribute::Bold),
             Cell::new("This is a green text").fg(Color::Green),
             Cell::new("This one has black background").bg(Color::Black),
        ])
        .add_row(vec![
            Cell::new("Blinky boi").add_attribute(Attribute::SlowBlink),
            Cell::new("This table's content is dynamically arranged. The table is exactly 80 characters wide.\nHere comes a reallylongwordthatshoulddynamicallywrap"),
            Cell::new("COMBINE ALL THE THINGS")
                .fg(Color::Green)
                .bg(Color::Black)
                .add_attributes(vec![
                    Attribute::Bold,
                    Attribute::SlowBlink,
                ])
        ]);

    println!("{table}");
}
