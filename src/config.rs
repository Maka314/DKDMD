use crate::models::ModelConfig;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

/// 配置结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub models: std::collections::HashMap<String, ModelConfig>,
    #[serde(default)]
    pub tool_bindings: std::collections::HashMap<String, String>,
}

/// 获取配置目录
pub fn get_config_dir() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("", "DKDMD", "DKDMD")?;
    Some(dirs.config_dir().to_path_buf())
}

/// 获取配置文件路径
pub fn get_config_path() -> Option<PathBuf> {
    let config_dir = get_config_dir()?;
    Some(config_dir.join("config.json"))
}

/// 加载配置
pub fn load_config() -> Config {
    match get_config_path() {
        Some(path) => {
            if path.exists() {
                match fs::read_to_string(&path) {
                    Ok(content) => match serde_json::from_str(&content) {
                        Ok(config) => config,
                        Err(_) => create_default_config(&path),
                    },
                    Err(_) => create_default_config(&path),
                }
            } else {
                create_default_config(&path)
            }
        }
        None => create_default_config(&get_config_dir().unwrap()),
    }
}

/// 创建默认配置
pub fn create_default_config(path: &PathBuf) -> Config {
    let config_dir = path.parent().unwrap();
    fs::create_dir_all(config_dir).ok();

    let default_config = Config {
        models: std::collections::HashMap::new(),
        tool_bindings: std::collections::HashMap::new(),
    };

    if let Err(e) = fs::write(path, serde_json::to_string_pretty(&default_config).unwrap()) {
        eprintln!("⚠️  创建默认配置文件失败: {}", e);
    }

    default_config
}

/// 保存配置
pub fn save_config(config: &Config) -> io::Result<()> {
    if let Some(path) = get_config_path() {
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(&path, serde_json::to_string_pretty(config).unwrap())?;
    }
    Ok(())
}