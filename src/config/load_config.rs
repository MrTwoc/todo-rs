// use std::collections::HashMap;

use std::{fs, path::Path};

use config::{Config, File};
use owo_colors::OwoColorize;
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
/// 指令：config 配置字段 配置值
/// 示例：config if_login true
pub fn save_config(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        return Err("参数不足，使用方法: config <配置字段> <配置值>".into());
    }

    let config_path = "app_config.toml";

    // 加载当前配置
    let mut current_config = load_config()?;

    // 获取字段名和值
    let field = args[1];
    let value = args[2];

    // 根据字段名更新配置
    match field {
        "if_login" => {
            current_config.if_login = value
                .parse::<bool>()
                .map_err(|_| "配置值错误: if_login 必须是 true 或 false")?;
        }
        "test" => {
            current_config.test = value.to_string();
        }
        "test2" => {
            current_config.test2 = value
                .parse::<u8>()
                .map_err(|_| "配置值错误: test2 必须是 0-255 之间的数字")?;
        }
        _ => return Err(format!("未知配置字段: {}", field).into()),
    }

    // 将更新后的配置写入文件
    let toml_content =
        toml::to_string(&current_config).map_err(|e| format!("配置序列化失败: {}", e))?;

    std::fs::write(config_path, toml_content).map_err(|e| format!("配置文件写入失败: {}", e))?;

    info!("配置已更新: {} = {}", field, value);
    println!("配置更新成功: {} = {}", field.green(), value.green());

    Ok(())
}
