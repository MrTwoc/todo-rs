use std::io::{self, Write};

use owo_colors::OwoColorize;
use todo_rs::{cmd_handler, help, logger, todors_init};
use tracing::*;

fn main() {
    let _guard = logger::init_logger();

    let _ = todors_init::init();
    info!("程序启动");
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
                print!("{}", &help::HELP_INFO);
                info!("input: help");
            }
            _ => {
                // 进入命令处理阶段，处理task和user相关指令
                if let Err(e) = cmd_handler::command_handle(&input) {
                    eprintln!("命令处理失败: {}", e);
                }
            }
        }
    }
}
