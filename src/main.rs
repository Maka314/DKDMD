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
                    eprintln!("❌ Error: config mode requires --model and --base-url");
                    std::process::exit(1);
                }
            } else {
                eprintln!("❌ Error: config mode requires --model");
                std::process::exit(1);
            }
        }
        "run" | "start" | "launch" => {
            command_mode_cli(&cli)?;
        }
        _ => {
            eprintln!("❌ Unknown command: {}", cli.command);
            eprintln!("Available commands: menu, config, run");
            std::process::exit(1);
        }
    }

    Ok(())
}