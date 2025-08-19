use std::io::{self, Write};

use owo_colors::OwoColorize;
use todo_rs::{about, cmd_handler};

fn main() {
    // 前期临时用print打印，后期想改为BufWriter，不知道能否进一步降低内存和CPU占用
    print!("{}\n", &about::PRINT_TITLE.green());
    print!("{}\n", &about::TITLE_INFO.green());

    // 基础删除线
    // println!("{}", "已取消的任务".strikethrough());
    // 组合样式（红色文字+删除线）
    // println!("{}", "失效条目".red().strikethrough());
    // let test = format!("{}", "1234你好abcd".yellow().bold().strikethrough());
    // let test = format!("{}", "1234你好abcd".strikethrough());
    // println!("{}", test);
    // println!(
    //     "{}",
    //     "风格: 任务完成(灰色+删除线)"
    //         .truecolor(150, 150, 150)
    //         .strikethrough()
    // );
    // println!("{}", "高级灰".truecolor(180, 180, 180).strikethrough());

    run();
}

fn run() {
    let mut input = String::with_capacity(128); // 预分配缓冲区

    loop {
        print!("{}> ", "------------------------------\n".green());
        std::io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("命令输入失败");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "exit" | "quit" | "q" => {
                println!("正在退出程序");
                std::process::exit(0);
            }
            "help" => {
                print!("{}", &about::HELP_INFO);
            }
            _ => {
                // 进入命令处理阶段
                if let Err(e) = cmd_handler::command_handle(&input) {
                    eprintln!("命令处理失败: {}", e);
                }
            }
        }
    }
}
