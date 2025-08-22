use std::{error::Error, fs, io};

use crate::task_module::Target;

pub struct TaskStorage;
impl TaskStorage {
    pub fn save(task: &[Target]) -> Result<(), Box<dyn Error>> {
        let file = fs::File::create("task.json")?;
        let writer = io::BufWriter::new(file);
        serde_json::to_writer(writer, task)?;
        Ok(())
    }

    // 从json文件中读取电影列表
    pub fn read() -> Result<Vec<Target>, Box<dyn Error>> {
        match fs::File::open("task.json") {
            Ok(f) => {
                let reader = io::BufReader::new(f);
                match serde_json::from_reader(reader) {
                    Ok(task) => Ok(task),
                    Err(e) if e.is_eof() => Ok(Vec::new()),
                    Err(e) => {
                        println!("读取文件失败: {}", e);
                        Err(e.into())
                    }
                }
            }
            // 优化: 文件不存在时不打印错误,直接返回空列表
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(e) => {
                println!("读取文件失败: {}", e);
                Err(e.into())
            }
        }
    }
}
