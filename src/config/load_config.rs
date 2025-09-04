// use std::collections::HashMap;

use config::Config;

pub fn load_config() {
    let _settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("todo_config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    // Print out our settings (as a HashMap)
    // println!("应用配置：");
    // println!(
    //     "{:?}",
    //     settings
    //         .try_deserialize::<HashMap<String, String>>()
    //         .unwrap()
    // );
}
