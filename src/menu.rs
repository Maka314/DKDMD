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
        eprintln!("\n\n👋 感谢使用 DKDMD!");
        std::process::exit(0);
    })
    .expect("无法设置 Ctrl+C handler");

    while running {
        clear_screen();
        println!("╔═════════════════════════════════════════╗");
        println!("║      任意门 - 本地 AI 工具管理器        ║");
        println!("╚═════════════════════════════════════════╝\n");

        let menu_options = vec![
            "📋 添加/管理模型",
            "🛠️  启动工具",
            "⚙️  显示配置",
            "🗑️  清除配置",
            "👋 退出",
        ];

        match Select::new("请选择操作:", menu_options).prompt() {
            Ok("📋 添加/管理模型") => {
                handle_model_management(&mut config);
            }
            Ok("🛠️  启动工具") => {
                handle_tool_launch(&mut config);
            }
            Ok("⚙️  显示配置") => {
                println!("\n📋 当前配置:");
                println!("{:#?}", config);
                let _ = Text::new("按回车键继续...").prompt();
            }
            Ok("🗑️  清除配置") => {
                if let Ok(true) = Confirm::new("⚠️  确定要清除所有配置吗?")
                    .with_default(false)
                    .prompt()
                {
                    config.models.clear();
                    save_config(&config).ok();
                    println!("✅ 配置已清除");
                    let _ = Text::new("按回车键继续...").prompt();
                }
            }
            Ok("👋 退出") => {
                running = false;
                println!("\n👋 感谢使用 DKDMD!");
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
    options.push("  ➕ 添加新模型".to_string());

    match Select::new("选择要管理的模型:", options).prompt() {
        Ok(selected) => {
            if selected == "  ➕ 添加新模型" {
                match Text::new("输入模型名称:").prompt() {
                    Ok(model_name) if !model_name.is_empty() => {
                        match Text::new("输入 Base URL:").prompt() {
                            Ok(base_url) if !base_url.is_empty() => {
                                let api_key = match Text::new("输入 API Key (可选, 直接回车跳过):").prompt() {
                                    Ok(key) if !key.is_empty() => Some(key),
                                    _ => None,
                                };
                                add_model_config(config, model_name, base_url, api_key);
                                save_config(config).ok();
                                println!("✅ 模型已添加");
                                let _ = Text::new("按回车键继续...").prompt();
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
        println!("❌ 没有可用模型，请先添加模型");
        let _ = Text::new("按回车键继续...").prompt();
        return;
    }

    let tools = [
        ("ClaudeCode", "claude"),
        ("Codex", "codex"),
    ];

    let tool_options: Vec<String> = tools
        .iter()
        .map(|(display_name, key)| {
            let bin_name = if *key == "codex" { "codegen" } else { *key };
            let installed = which_exists(bin_name);
            let status = if !installed {
                "未安装".to_string()
            } else {
                config
                    .tool_bindings
                    .get(*key)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "未设置".to_string())
            };
            format!("  {} ({})", display_name, status)
        })
        .collect();

    match Select::new("选择要启动的工具:", tool_options).prompt() {
        Ok(selected) => {
            let selected_tool = selected.trim();
            let tool_name = if selected_tool.starts_with("ClaudeCode") {
                "claude"
            } else {
                "codex"
            };

            let model_names: Vec<String> = config
                .models
                .iter()
                .map(|(name, _)| format!("  {}", name))
                .collect();

            match Select::new("选择要使用的模型:", model_names).prompt() {
                Ok(model_selected) => {
                    let model_name = model_selected.trim().to_string();
                    config
                        .tool_bindings
                        .insert(tool_name.to_string(), model_name.clone());
                    save_config(config).ok();

                    if let Err(e) = run_tool(config, tool_name, &model_name) {
                        eprintln!("❌ 错误: {}", e);
                    }
                    let _ = Text::new("按回车键继续...").prompt();
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
}