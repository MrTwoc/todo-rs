use std::error::Error;

pub fn login() -> Result<(), Box<dyn Error>> {
    println!("登录成功");

    Ok(())
}
