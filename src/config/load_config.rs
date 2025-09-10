// use std::collections::HashMap;

use std::{fs, path::Path};

use config::{Config, File};
use tracing::info;

use crate::config::config::AppConfig;

pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = "app_config.toml";

    // 默认配置
    let default_config = AppConfig {
        if_login: false,          // 是否需要登录
        test: "test".to_string(), // 测试用
        test2: 123,               // 测试用
    };

    // 确保配置文件存在
    if !Path::new(config_path).exists() {
        fs::write(config_path, toml::to_string(&default_config).unwrap())?;
        info!("配置文件不存在，已创建默认配置文件")
    }

    let config = Config::builder()
        .add_source(File::with_name("app_config"))
        .build()?
        .try_deserialize::<AppConfig>()?;
    // .map_err(|e| e.into())
    Ok(config)
}

/// 保存配置,可在运行时修改配置
pub fn save_config() {
    todo!()
}
