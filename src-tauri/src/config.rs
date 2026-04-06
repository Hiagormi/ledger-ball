use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_parse_mode")]
    pub parse_mode: String,

    #[serde(default = "default_ai_provider")]
    pub ai_provider: String,

    #[serde(default)]
    pub api_key: String,

    #[serde(default = "default_api_base")]
    pub api_base: String,

    #[serde(default = "default_model")]
    pub model: String,
}

fn default_parse_mode() -> String { "local".to_string() }
fn default_ai_provider() -> String { "deepseek".to_string() }
fn default_api_base() -> String { "https://api.deepseek.com/v1".to_string() }
fn default_model() -> String { "deepseek-chat".to_string() }

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            parse_mode: default_parse_mode(),
            ai_provider: default_ai_provider(),
            api_key: String::new(),
            api_base: default_api_base(),
            model: default_model(),
        }
    }
}

/// 获取配置文件路径 (存储在项目目录的 data 文件夹下)
pub fn get_config_path() -> Result<PathBuf, String> {
    // 获取当前可执行文件的目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("无法获取程序目录: {}", e))?
        .parent()
        .ok_or("无法获取父目录")?
        .to_path_buf();

    // 创建 data 目录
    let data_dir = exe_dir.join("data");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("无法创建数据目录: {}", e))?;
    }

    Ok(data_dir.join("config.json"))
}