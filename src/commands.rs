use crate::config::{load_config, save_config};
use crate::models::add_model_config;
use crate::tools::run_tool;
use crate::cli::Cli;

/// 配置模式
pub fn config_mode_cli(model_name: &str, path: &str, port: Option<u16>) {
    let mut config = load_config();
    add_model_config(&mut config, model_name.to_string(), path.to_string(), port);
    save_config(&config).ok();
}

/// 命令模式启动工具
pub fn command_mode_cli(cli: &Cli) -> anyhow::Result<()> {
    let config = load_config();

    if let Some(tool) = &cli.tool {
        if let Some(model) = &cli.model {
            run_tool(&config, tool, model)?;
        } else {
            eprintln!("❌ 错误: 命令模式需要 --model 参数");
            return Err(anyhow::anyhow!("缺少 --model 参数"));
        }
    } else {
        eprintln!("❌ 错误: 命令模式需要 --tool 参数");
        return Err(anyhow::anyhow!("缺少 --tool 参数"));
    }

    Ok(())
}