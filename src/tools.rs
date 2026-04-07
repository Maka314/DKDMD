use crate::config::Config;
use crate::models::get_model_config;
use std::process::{Command, Stdio};

/// 启动工具
pub fn run_tool(config: &Config, tool_name: &str, model_name: &str) -> anyhow::Result<()> {
    if let Some(model_config) = get_model_config(config, model_name) {
        let api_url = match &model_config.api_url {
            Some(url) => url.clone(),
            None => {
                eprintln!("⚠️  模型 {} 没有配置 API URL", model_name);
                return Err(anyhow::anyhow!("模型未配置 API URL"));
            }
        };

        // 在命令模式下，使用默认的 codegen 工具
        let tool_program = if tool_name == "codex" {
            "codegen".to_string()
        } else if tool_name == "claudecode" {
            "claudecode".to_string()
        } else {
            tool_name.to_string()
        };

        let mut cmd = Command::new(&tool_program);

        // 替换 API URL 占位符
        let api_url = api_url.replace("{API_URL}", model_config.path.as_str());
        cmd.arg("--api-url").arg(&api_url);

        if let Some(port) = model_config.port {
            cmd.arg("--port").arg(port.to_string());
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