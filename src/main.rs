use std::io::{self, Write};

use chrono::Local;
use owo_colors::OwoColorize;
use todo_rs::{about, cmd_handler};
use tracing::*;
use tracing_subscriber::fmt::format::Writer;
// use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::{self, fmt::time::FormatTime};

// 用来格式化日志的输出时间格式
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}

fn main() {
    // 直接初始化，采用默认的Subscriber，默认只输出INFO、WARN、ERROR级别的日志
    // tracing_subscriber::fmt::init();

    // 使用tracing_appender，指定日志的输出目标位置
    // 参考: https://docs.rs/tracing-appender/0.2.0/tracing_appender/
    // 如果不是在main函数中，guard必须返回到main()函数中，否则不输出任何信息到日志文件
    let file_appender = tracing_appender::rolling::daily("./logs", "tracing.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置、设置日志的时间格式
    // 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);

    // 初始化并设置日志格式(定制和筛选日志)
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        // .with_writer(io::stdout.and(non_blocking)) // 写入文件，将覆盖上面的标准输出
        .with_writer(non_blocking) // 只保留文件写入器
        .with_ansi(false) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
        .event_format(format)
        .init(); // 初始化并将SubScriber设置为全局SubScriber

    // trace!("tracing-trace");
    // debug!("tracing-debug");
    // info!("tracing-info");
    // warn!("tracing-warn");
    // error!("tracing-error");

    // 前期临时用print打印，后期想改为BufWriter，不知道能否进一步降低内存和CPU占用
    println!("{}", &about::PRINT_TITLE.green());
    println!("{}", &about::TITLE_INFO.green());

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
                info!("input: help");
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
