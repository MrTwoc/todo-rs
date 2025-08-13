pub fn command_handle(input: &str) {
    let args: Vec<&str> = input.trim().split_whitespace().collect();
    println!("接收参数: {:?}", args);
}
