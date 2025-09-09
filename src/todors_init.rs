// use std::collections::HashMap;

use owo_colors::OwoColorize;
use tracing::error;
// use tracing::error;

use crate::{
    config::{self, config::AppConfig},
    help,
    user::{self, user::User},
};
use config::load_config;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // let _guard = logger::init_logger();

    if let Err(e) = User::init() {
        error!("用户初始化失败: {:?}", e);
    }
    // info!("用户初始化完成");
    match load_config::load_config() {
        Ok(_) => {
            let config = load_config::load_config()?;

            let if_login = config.get::<bool>("if_login").unwrap_or(false);

            let _app_config = AppConfig { if_login: if_login };
            // println!("if_login:{:?}", &_app_config.if_login);
            if _app_config.if_login {
                user::user_modules::user_login()?;
            }
        }
        Err(e) => {
            error!("配置文件初始化失败: {:?}", e);
        }
    }

    println!("{}", &help::PRINT_TITLE.green());
    println!("{}", &help::TITLE_INFO.green());
    Ok(())
}
