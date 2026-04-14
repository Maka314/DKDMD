use clap::Parser;

/// DKDMD - 本地 AI 工具管理器
#[derive(Parser)]
#[command(name = "dkdmd")]
#[command(author = "DKDMD Team")]
#[command(about = "本地 AI 工具管理器 - 配置和管理本地 AI 工具", long_about = None)]
pub struct Cli {
    /// 模型名称
    #[arg(short, long)]
    pub model: Option<String>,

    /// 工具名称
    #[arg(short, long)]
    pub tool: Option<String>,

    /// Base URL
    #[arg(short, long)]
    pub base_url: Option<String>,

    /// 命令模式
    #[arg(long, default_value = "menu")]
    pub command: String,
}