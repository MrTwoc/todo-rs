use std::{error::Error, io};

use tracing::info;

use crate::{
    config::load_config::load_config, user_module::user::OnineUser, user_module::user::User,
};

pub fn user_login() -> Result<(), Box<dyn Error>> {
    // 初始化在线用户列表
    let mut online_user = OnineUser::new();

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

                // 登录成功后，将用户加入在线用户列表
                online_user.add_online_user(user_pwd.id, true, username, user_pwd.level);

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
