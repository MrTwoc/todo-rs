use std::io::{self, Write};

use todo_rs::{about, command};

const HELP_INFO: &str = r#"
'help'将显示帮助信息
'exit'/'quit'/'q'将退出程序
"#;
fn main() {
    print!("{}\n", about::PRINT_TITLE);
    print!("---------------------------\n");
    print!("{}\n", &HELP_INFO);

    run();
}

fn run() {
    let mut input = String::with_capacity(128); // 预分配缓冲区

    loop {
        print!("> ");
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
                print!("{}", &HELP_INFO);
            }
            _ => {
                command::command_handle(&input);
            }
        }
    }
}
