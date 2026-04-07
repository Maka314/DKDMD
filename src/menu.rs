use crate::config::{load_config, save_config};
use crate::models::add_model_config;
use crate::tools::run_tool;
use std::io::{self, Write};
use std::thread;

/// 清屏
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

/// 显示主菜单
pub fn show_menu() {
    let config = load_config();

    clear_screen();
    println!("╔═══════════════════════════════════════╗");
    println!("║      DKDMD - 本地 AI 工具管理器        ║");
    println!("╚════════════════════════════════════════╝\n");

    println!("📋 模型配置 ({})\n", config.models.len());
    for (name, _) in &config.models {
        println!("  [1] {}", name);
    }

    if config.models.is_empty() {
        println!("  [?] 添加模型");
    }
    println!("\n");

    println!("🛠️  启动工具 ({})\n", config.models.len());
    for (name, _) in &config.models {
        println!("  [2] {}", name);
    }

    if config.models.is_empty() {
        println!("  [?] 添加工具");
    }
    println!("\n");

    println!("⚙️  系统管理\n");
    println!("  [3] 显示配置");
    println!("  [4] 清除配置");
    println!("  [5] 退出\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    print!("请选择操作: ");
    io::stdout().flush().unwrap();
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
        show_menu();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("读取输入失败");

        match choice.trim() {
            "1" => {
                // 添加/选择模型
                println!("\n当前可用模型:");
                for (name, _) in &config.models {
                    println!("  - {}", name);
                }
                print!("\n输入模型名称: ");
                io::stdout().flush().unwrap();
                let mut model_name = String::new();
                io::stdin().read_line(&mut model_name).expect("读取输入失败");

                let model_name = model_name.trim().to_string();
                if model_name.is_empty() {
                    continue;
                }

                print!("输入模型路径 (或 API URL): ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("读取输入失败");
                let path = path.trim().to_string();

                print!("输入端口 (可选, 直接回车跳过): ");
                io::stdout().flush().unwrap();
                let mut port_input = String::new();
                io::stdin().read_line(&mut port_input).expect("读取输入失败");

                let port = if port_input.trim().is_empty() {
                    None
                } else {
                    port_input.trim().parse().ok()
                };

                add_model_config(&mut config, model_name, path, port);
            }
            "2" => {
                // 启动工具
                println!("\n当前可用工具:");
                for (name, _) in &config.models {
                    println!("  - {}", name);
                }
                print!("\n输入工具名称: ");
                io::stdout().flush().unwrap();
                let mut tool_name = String::new();
                io::stdin().read_line(&mut tool_name).expect("读取输入失败");

                let tool_name = tool_name.trim().to_string();
                if tool_name.is_empty() {
                    continue;
                }

                print!("输入使用的模型: ");
                io::stdout().flush().unwrap();
                let mut model_name = String::new();
                io::stdin().read_line(&mut model_name).expect("读取输入失败");

                let model_name = model_name.trim().to_string();
                if model_name.is_empty() {
                    continue;
                }

                if let Err(e) = run_tool(&config, &tool_name, &model_name) {
                    eprintln!("❌ 错误: {}", e);
                }
                println!("\n按回车键继续...");
                io::stdin().read_line(&mut choice).expect("读取输入失败");
            }
            "3" => {
                // 显示配置
                println!("\n📋 当前配置:");
                println!("{:#?}", config);
                println!("\n按回车键继续...");
                io::stdin().read_line(&mut choice).expect("读取输入失败");
            }
            "4" => {
                // 清除配置
                print!("⚠️  确定要清除所有配置吗? (y/N): ");
                io::stdout().flush().unwrap();
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).expect("读取输入失败");

                if confirm.trim().to_lowercase() == "y" {
                    config.models.clear();
                    save_config(&config).ok();
                    eprintln!("✅ 配置已清除");
                    println!("按回车键继续...");
                    io::stdin().read_line(&mut choice).expect("读取输入失败");
                }
            }
            "5" => {
                running = false;
                eprintln!("\n👋 感谢使用 DKDMD!");
            }
            _ => {
                eprintln!("无效选择，请重试");
                thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }
}