use serde::{Deserialize, Serialize};

/// 模型配置
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    #[serde(alias = "path")]
    pub base_url: String,
    #[serde(default, alias = "api_url")]
    pub api_key: Option<String>,
}

/// 添加模型配置
pub fn add_model_config(
    config: &mut crate::config::Config,
    name: String,
    base_url: String,
    api_key: Option<String>,
) {
    config.models.insert(
        name.clone(),
        ModelConfig {
            name,
            base_url,
            api_key,
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