use std::{error::Error, fs, io, sync::RwLock};

use tracing::error;

use crate::task_module::Target;

/// 任务缓存
static TASKS_CACHE: RwLock<Option<Vec<Target>>> = RwLock::new(None);

pub struct TaskStorage;
impl TaskStorage {
    pub fn save(task: &[Target]) -> Result<(), Box<dyn Error>> {
        let file = fs::File::create("task.json")?;
        let writer = io::BufWriter::new(file);
        serde_json::to_writer(writer, task)?;

        *TASKS_CACHE.write().unwrap() = Some(task.to_vec());
        Ok(())
    }

    // 从json文件中读取任务列表
    pub fn read() -> Result<Vec<Target>, Box<dyn Error>> {
        // 检查缓存
        if let Some(cache) = TASKS_CACHE.read().unwrap().as_ref() {
            return Ok(cache.clone());
        }

        // 尝试读取文件
        let tasks = match fs::File::open("task.json") {
            Ok(f) => {
                let reader = io::BufReader::new(f);
                match serde_json::from_reader(reader) {
                    Ok(task) => task,
                    Err(e) if e.is_eof() => Vec::new(),
                    Err(e) => {
                        // 使用日志框架记录错误
                        error!("JSON解析失败: {}", e);
                        return Err(e.into());
                    }
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                // 文件不存在时返回空列表
                Vec::new()
            }
            Err(e) => {
                // 使用日志框架记录错误
                error!("文件打开失败: {}", e);
                return Err(e.into());
            }
        };

        // 仅在成功时更新缓存
        *TASKS_CACHE.write().unwrap() = Some(tasks.clone());
        Ok(tasks)
    }
}
