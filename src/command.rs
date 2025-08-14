use owo_colors::OwoColorize;

pub fn command_handle(input: &str) {
    let args: Vec<&str> = input.trim().split_whitespace().collect();

    if args.is_empty() {
        return;
    }

    // println!("接收参数: {:?}", args);
    match args[0] {
        "add" => println!("add命令"),
        "list" => println!("list命令"),
        "edit" => println!("edit命令"),
        "del" => println!("del命令"),
        "sysinfo" => {
            command_sysinfo();
        }
        _ => println!("{} > {}", "未知命令".red(), args[0]),
    }
}

fn command_sysinfo() {
    // let mut sys = sysinfo::System::new_all();
    let mut sys = sysinfo::System::new();
    // sys.refresh_all();
    sys.refresh_cpu_usage();
    sys.refresh_memory();

    if let Some(process) = sys.process(sysinfo::Pid::from(std::process::id() as usize)) {
        println!("内存使用 > {} MB", process.memory() / 1024 / 1024);
        println!("CPU使用率 > {:.1}%", process.cpu_usage());
    }
}
