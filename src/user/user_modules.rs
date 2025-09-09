use std::{error::Error, io};

use tracing::info;

use crate::{config::load_config::load_config, user::user::User};

/// 登陆功能，检测 if_login 状态
/// 为true, 提示输入用户名密码，
/// 为false, 跳过
// pub fn user_login() -> Result<(), Box<dyn Error>> {
//     let config = load_config()?;
//     let if_login = config.get_bool("if_login")?;

//     if if_login {
//         loop {
//             // 提示用户输出 用户名,
//             // 根据用户名查找用户，返回User，再提示输入密码，
//             // 将User内密码与输入密码对比
//             println!("请输入用户名 >");
//             let mut input = String::new();
//             io::stdin().read_line(&mut input).expect("输入失败");

//             let input = input.trim().to_lowercase().replace(" ", "");

//             match User::find(&input) {
//                 Ok(user) => {
//                     // 密码验证
//                     println!("请输入密码 >");
//                     let mut input = String::new();
//                     io::stdin().read_line(&mut input).expect("输入失败");

//                     let input = input.trim().to_lowercase().replace(" ", "");
//                     if input == user.password {
//                         println!("登录成功");
//                         info!("user:{},Login Success", &user.name);
//                         break;
//                     } else {
//                         println!("密码错误,请重新输入");
//                         continue;
//                     }
//                 }
//                 Err(_) => {
//                     println!("用户不存在,请重新输入");
//                     continue;
//                 }
//             }
//         }
//     }
//     Ok(())
// }

pub fn user_login() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    let if_login = config.get_bool("if_login")?;

    if if_login == true {
        // 用户名验证循环
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
