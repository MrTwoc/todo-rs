use std::{collections::HashMap, error::Error, io};

use tracing::{info, warn};

use crate::{
    config::load_config::load_config,
    user_module::user::{LoginInfo, OnlineUser, User},
};

use std::sync::{OnceLock, RwLock};
static ONLINE_USERS: OnceLock<RwLock<OnlineUser>> = OnceLock::new();

pub fn init_online_users() {
    ONLINE_USERS.get_or_init(|| {
        RwLock::new(OnlineUser {
            user_info: HashMap::new(),
        })
    });
}

pub fn get_online_users() -> &'static RwLock<OnlineUser> {
    ONLINE_USERS.get_or_init(|| {
        warn!("OnlineUsers 未初始化,使用默认值");
        RwLock::new(OnlineUser {
            user_info: HashMap::new(),
        })
    })
}

pub fn add_online_user(user: &User) {
    let mut guard = get_online_users().write().unwrap();
    guard.user_info.insert(
        user.id,
        LoginInfo {
            username: user.name.clone(),
            user_level: user.level,
        },
    );
}

pub fn user_login() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    let if_login = config.if_login;

    if if_login == true {
        /*
        用户名验证循环（最多尝试3次）
        */
        let username = loop {
            println!("请输入用户名 >");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("输入失败");
            let input = input.trim().replace(' ', "");

            match User::find(&input) {
                Ok(user) => break user.name,
                Err(_) => println!("用户不存在，请重新输入"),
            }
        };

        // 密码验证循环（最多尝试3次）
        let mut attempts = 0;
        let user_pwd = User::find(&username)?;
        loop {
            println!("请输入密码 >");
            let mut password_input = String::new();
            io::stdin()
                .read_line(&mut password_input)
                .expect("输入失败");
            let password_input = password_input.trim().replace(' ', "");

            if password_input == user_pwd.password {
                println!("登录成功");
                info!("User {} Login Success", username);
                add_online_user(&user_pwd);

                break;
            } else {
                attempts += 1;
                if attempts >= 3 {
                    println!("尝试次数过多，退出程序");
                    std::process::exit(1);
                }
                println!("密码错误，剩余尝试次数: {}", 3 - attempts);
            }
        }
    }
    Ok(())
}
