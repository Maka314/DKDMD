use clap::Parser;

/// DKDMD - 本地 AI 工具管理器
#[derive(Parser)]
#[command(name = "dkdmd")]
#[command(author = "DKDMD Team")]
#[command(about = "本地 AI 工具管理器 - 配置和管理本地 AI 工具", long_about = None)]
pub struct Cli {
    /// 模型名称或路径
    #[arg(short, long)]
    pub model: Option<String>,

    /// 工具名称
    #[arg(short, long)]
    pub tool: Option<String>,

    /// 端口号
    #[arg(short, long)]
    pub port: Option<u16>,

    /// 命令模式
    #[arg(long, default_value = "menu")]
    pub command: String,

    /// 配置文件路径
    #[arg(short, long)]
    pub config: Option<std::path::PathBuf>,
}