use crate::config::{load_config, save_config, Config};
use crate::models::add_model_config;
use crate::tools::run_tool;
use inquire::{Select, Text, Confirm};
use std::process::Command;

fn which_exists(program: &str) -> bool {
    Command::new("which")
        .arg(program)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// 清屏
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

/// 菜单模式
pub fn menu_mode() {
    let mut config = load_config();
    let mut running = true;

    ctrlc::set_handler(|| {
        eprintln!("\n\n👋 Goodbye!");
        std::process::exit(0);
    })
    .expect("Failed to set Ctrl+C handler");

    while running {
        clear_screen();
        println!("╔═════════════════════════════════════════╗");
        println!("║         DKDMD - AI Tool Manager         ║");
        println!("╚═════════════════════════════════════════╝\n");

        let menu_options = vec![
            "📋 Manage Models",
            "🛠️  Launch Tool",
            "⚙️  Show Config",
            "🗑️  Clear Config",
            "👋 Exit",
        ];

        match Select::new("Select an action:", menu_options).prompt() {
            Ok("📋 Manage Models") => {
                handle_model_management(&mut config);
            }
            Ok("🛠️  Launch Tool") => {
                handle_tool_launch(&mut config);
            }
            Ok("⚙️  Show Config") => {
                println!("\n⚙️  Configuration\n");
                if config.models.is_empty() {
                    println!("  No models configured.");
                } else {
                    println!("  Models ({})", config.models.len());
                    println!("  {}", "─".repeat(38));
                    for (key, m) in &config.models {
                        let bound_tool = config.tool_bindings
                            .iter()
                            .filter(|(_, v)| v.as_str() == key)
                            .map(|(t, _)| t.as_str())
                            .collect::<Vec<_>>()
                            .join(", ");
                        println!("  🤖 {}", m.name);
                        println!("     Base URL : {}", m.base_url);
                        match &m.api_key {
                            Some(_) => println!("     API Key  : ••••••••"),
                            None    => println!("     API Key  : (none)"),
                        }
                        if !bound_tool.is_empty() {
                            println!("     Used by  : {}", bound_tool);
                        }
                        println!();
                    }
                }
                if !config.tool_bindings.is_empty() {
                    println!("  Tool Bindings");
                    println!("  {}", "─".repeat(38));
                    for (tool, model) in &config.tool_bindings {
                        println!("  🛠️  {:10} → {}", tool, model);
                    }
                    println!();
                }
                let _ = Text::new("Press Enter to continue...").prompt();
            }
            Ok("🗑️  Clear Config") => {
                if let Ok(true) = Confirm::new("⚠️  Clear all configuration?")
                    .with_default(false)
                    .prompt()
                {
                    config.models.clear();
                    config.tool_bindings.clear();
                    save_config(&config).ok();
                    println!("✅ Config cleared");
                    let _ = Text::new("Press Enter to continue...").prompt();
                }
            }
            Ok("👋 Exit") => {
                running = false;
                println!("\n👋 Goodbye!");
            }
            Err(_) => {
                running = false;
            }
            _ => {}
        }
    }
}

/// 处理模型管理
fn handle_model_management(config: &mut Config) {
    let model_options: Vec<String> = config
        .models
        .iter()
        .map(|(name, _)| format!("  {}", name))
        .collect();

    let mut options = model_options.clone();
    options.push("  ➕ Add new model".to_string());

    match Select::new("Select a model to manage:", options).prompt() {
        Ok(selected) => {
            if selected == "  ➕ Add new model" {
                match Text::new("Model name:").prompt() {
                    Ok(model_name) if !model_name.is_empty() => {
                        match Text::new("Base URL:").prompt() {
                            Ok(base_url) if !base_url.is_empty() => {
                                let api_key = match Text::new("API Key (optional, press Enter to skip):").prompt() {
                                    Ok(key) if !key.is_empty() => Some(key),
                                    _ => None,
                                };
                                add_model_config(config, model_name, base_url, api_key);
                                save_config(config).ok();
                                println!("✅ Model added");
                                let _ = Text::new("Press Enter to continue...").prompt();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(_) => {}
    }
}

/// 处理工具启动
fn handle_tool_launch(config: &mut Config) {
    if config.models.is_empty() {
        println!("❌ No models configured. Please add a model first.");
        let _ = Text::new("Press Enter to continue...").prompt();
        return;
    }

    let tools = [
        ("ClaudeCode", "claude",  "curl -fsSL https://claude.ai/install.sh | bash"),
        ("Codex",      "codex",   "npm i -g @openai/codex"),
    ];

    let tool_options: Vec<String> = tools
        .iter()
        .map(|(display_name, key, _install_cmd)| {
            let installed = which_exists(key);
            let status = if !installed {
                "not installed".to_string()
            } else {
                config
                    .tool_bindings
                    .get(*key)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "not configured".to_string())
            };
            format!("  {} ({})", display_name, status)
        })
        .collect();

    match Select::new("Select a tool to launch:", tool_options).prompt() {
        Ok(selected) => {
            let selected_tool = selected.trim();
            let (tool_name, install_cmd) = if selected_tool.starts_with("ClaudeCode") {
                ("claude", tools[0].2)
            } else {
                ("codex", tools[1].2)
            };

            // Tool not installed: show install command and prompt user
            if !which_exists(tool_name) {
                println!("\n⚠️  This tool is not installed. Install command:");
                println!("\n    {}\n", install_cmd);
                if let Ok(true) = Confirm::new("Run the install command now?")
                    .with_default(true)
                    .prompt()
                {
                    let status = std::process::Command::new("sh")
                        .args(["-c", install_cmd])
                        .stdin(std::process::Stdio::inherit())
                        .stdout(std::process::Stdio::inherit())
                        .stderr(std::process::Stdio::inherit())
                        .status();
                    match status {
                        Ok(s) if s.success() => {
                            println!("\n✅ Installation successful!");
                        }
                        _ => {
                            println!("\n❌ Installation failed. Run manually: {}", install_cmd);
                        }
                    }
                }
                let _ = Text::new("Press Enter to continue...").prompt();
                return;
            }

            let model_names: Vec<String> = config
                .models
                .iter()
                .map(|(name, _)| format!("  {}", name))
                .collect();

            match Select::new("Select a model:", model_names).prompt() {
                Ok(model_selected) => {
                    let model_name = model_selected.trim().to_string();
                    config
                        .tool_bindings
                        .insert(tool_name.to_string(), model_name.clone());
                    save_config(config).ok();

                    if let Err(e) = run_tool(config, tool_name, &model_name) {
                        eprintln!("❌ Error: {}", e);
                    }
                    let _ = Text::new("Press Enter to continue...").prompt();
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
}