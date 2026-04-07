use serde::{Deserialize, Serialize};

/// 模型配置
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub path: String,
    pub port: Option<u16>,
    pub api_url: Option<String>,
}

/// 添加模型配置
pub fn add_model_config(
    config: &mut crate::config::Config,
    name: String,
    path: String,
    port: Option<u16>,
) {
    config.models.insert(
        name.clone(),
        ModelConfig {
            name,
            path,
            port,
            api_url: None,
        },
    );
}

/// 获取模型配置
pub fn get_model_config<'a>(
    config: &'a crate::config::Config,
    name: &str,
) -> Option<&'a ModelConfig> {
    config.models.get(name)
}