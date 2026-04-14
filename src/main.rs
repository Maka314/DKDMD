mod cli;
mod commands;
mod config;
mod models;
mod menu;
mod tools;

use clap::Parser;
use commands::{command_mode_cli, config_mode_cli};
use crate::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command.as_str() {
        "menu" => {
            menu::menu_mode();
        }
        "config" => {
            if let Some(model) = &cli.model {
                if let Some(base_url) = &cli.base_url {
                    config_mode_cli(model, base_url, cli.api_key.as_deref());
                } else {
                    eprintln!("❌ 错误: 配置模式需要 --model 和 --base-url 参数");
                    std::process::exit(1);
                }
            } else {
                eprintln!("❌ 错误: 配置模式需要 --model 参数");
                std::process::exit(1);
            }
        }
        "run" | "start" | "launch" => {
            command_mode_cli(&cli)?;
        }
        _ => {
            eprintln!("❌ 未知命令: {}", cli.command);
            eprintln!("可用命令: menu, config, run");
            std::process::exit(1);
        }
    }

    Ok(())
}