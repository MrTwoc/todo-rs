use owo_colors::OwoColorize;

use crate::{about, logger, user::user::User};

pub fn init() {
    logger::init_logger();

    User::init();

    // 前期临时用print打印，后期想改为BufWriter，不知道能否进一步降低内存和CPU占用
    println!("{}", &about::PRINT_TITLE.green());
    println!("{}", &about::TITLE_INFO.green());
}
