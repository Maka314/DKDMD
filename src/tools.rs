use crate::config::Config;
use crate::models::get_model_config;
use std::process::{Command, Stdio};

/// 启动工具
pub fn run_tool(config: &Config, tool_name: &str, model_name: &str) -> anyhow::Result<()> {
    if let Some(model_config) = get_model_config(config, model_name) {
        // 在命令模式下，使用默认的 codegen 工具
        let tool_program = if tool_name == "codex" {
            "codex".to_string()
        } else if tool_name == "claude" {
            "claude".to_string()
        } else {
            tool_name.to_string()
        };

        let mut cmd = Command::new(&tool_program);

        // Claude Code 通过环境变量配置 API 端点和模型
        if tool_name == "claude" {
            cmd.env("ANTHROPIC_BASE_URL", &model_config.base_url);
            if let Some(api_key) = &model_config.api_key {
                cmd.env("ANTHROPIC_API_KEY", api_key);
            }
            cmd.arg("--model").arg(&model_config.name);
        } else if tool_name == "codex" {
            cmd.env("OPENAI_BASE_URL", &model_config.base_url);
            if let Some(api_key) = &model_config.api_key {
                cmd.env("OPENAI_API_KEY", api_key);
            }
            cmd.arg("--model").arg(&model_config.name);
        }

        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        let mut child = cmd.spawn()?;

        eprintln!("\n🚀 启动工具: {}", tool_program);
        eprintln!("🔗 使用模型: {}", model_name);
        eprintln!("🖥️  进程 PID: {}\n", child.id());

        // 等待进程退出
        match child.wait() {
            Ok(status) => {
                if status.success() {
                    eprintln!("\n✅ 工具已正常退出");
                } else {
                    eprintln!("\n⚠️  工具退出状态: {:?}", status);
                }
            }
            Err(e) => {
                eprintln!("\n❌ 等待进程时出错: {}", e);
            }
        }

        Ok(())
    } else {
        eprintln!("⚠️  模型 {} 未配置", model_name);
        Err(anyhow::anyhow!("模型未配置"))
    }
}