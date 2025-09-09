// use std::collections::HashMap;

use std::{fs, path::Path};

use config::{Config, File};
// use tracing::info;

// pub fn config_init() -> Result<(), Box<dyn std::error::Error>> {
//     // 配置文件路径
//     let config_path = "app_config.toml";

//     // 如果文件不存在则创建
//     if !std::path::Path::new(config_path).exists() {
//         let default_config = r#"# 是否开启用户登陆
// if_login = false"#;
//         fs::write(config_path, default_config)?;
//     }
//     info!("配置文件初始化完成");
//     Ok(())
// }

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = "app_config.toml";

    // 确保配置文件存在
    if !Path::new(config_path).exists() {
        let default_config = "if_login = true";
        fs::write(config_path, default_config)?;
    }

    Config::builder()
        .add_source(File::with_name("app_config"))
        .build()
        .map_err(|e| e.into())
}

/// 保存配置,可在运行时修改配置
pub fn save_config() {
    todo!()
}
