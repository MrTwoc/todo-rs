use std::process::Command;

/*
    负责处理指令
*/

pub fn command_sysinfo() {
    let mut sys = sysinfo::System::new();
    sys.refresh_all();

    if let Some(process) = sys.process(sysinfo::Pid::from(std::process::id() as usize)) {
        println!(
            "内存使用 > {} MB\nCPU使用率 > {:.1}%",
            process.memory() / 1024 / 1024,
            process.cpu_usage()
        );
    }
}

pub fn command_clear() {
    // 在Windows上执行cls命令清空控制台
    if let Err(e) = Command::new("cmd").arg("/c").arg("cls").status() {
        eprintln!("清空控制台失败: {}", e);
    }
}
