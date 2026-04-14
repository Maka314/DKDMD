use crate::config::{load_config, save_config};
use crate::models::add_model_config;
use crate::tools::run_tool;
use crate::cli::Cli;

/// 配置模式
pub fn config_mode_cli(model_name: &str, base_url: &str, api_key: Option<&str>) {
    let mut config = load_config();
    add_model_config(&mut config, model_name.to_string(), base_url.to_string(), api_key.map(|s| s.to_string()));
    save_config(&config).ok();
}

/// 命令模式启动工具
pub fn command_mode_cli(cli: &Cli) -> anyhow::Result<()> {
    let config = load_config();

    if let Some(tool) = &cli.tool {
        if let Some(model) = &cli.model {
            run_tool(&config, tool, model)?;
        } else {
            eprintln!("❌ Error: run mode requires --model");
            return Err(anyhow::anyhow!("missing --model"));
        }
    } else {
        eprintln!("❌ Error: run mode requires --tool");
        return Err(anyhow::anyhow!("missing --tool"));
    }

    Ok(())
}