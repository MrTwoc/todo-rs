use owo_colors::OwoColorize;
use tracing::error;
// use tracing::error;

use crate::{config, help, user::user::User};
use config::load_config;

pub fn init() {
    // let _guard = logger::init_logger();

    if let Err(e) = User::init() {
        error!("用户初始化失败: {:?}", e);
    }
    // info!("用户初始化完成");
    load_config::load_config();

    // 前期临时用print打印，后期想改为BufWriter，不知道能否进一步降低内存和CPU占用
    println!("{}", &help::PRINT_TITLE.green());
    println!("{}", &help::TITLE_INFO.green());
}
