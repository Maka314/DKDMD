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

        eprintln!("\n🚀 Launching: {}", tool_program);
        eprintln!("🔗 Model: {}", model_name);
        eprintln!("🖥️  PID: {}\n", child.id());

        match child.wait() {
            Ok(status) => {
                if status.success() {
                    eprintln!("\n✅ Tool exited normally");
                } else {
                    eprintln!("\n⚠️  Tool exited with status: {:?}", status);
                }
            }
            Err(e) => {
                eprintln!("\n❌ Error waiting for process: {}", e);
            }
        }

        Ok(())
    } else {
        eprintln!("⚠️  Model '{}' not configured", model_name);
        Err(anyhow::anyhow!("model not configured"))
    }
}