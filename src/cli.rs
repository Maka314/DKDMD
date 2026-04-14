use clap::Parser;

/// DKDMD - Local AI Tool Manager
#[derive(Parser)]
#[command(name = "dkdmd")]
#[command(author = "DKDMD Team")]
#[command(about = "Dokodemo Doa - Connect local AI apps to any API endpoint", long_about = None)]
pub struct Cli {
    /// Model name
    #[arg(short, long)]
    pub model: Option<String>,

    /// Tool name
    #[arg(short, long)]
    pub tool: Option<String>,

    /// Base URL
    #[arg(short, long)]
    pub base_url: Option<String>,

    /// API Key
    #[arg(short = 'k', long)]
    pub api_key: Option<String>,

    /// Command mode
    #[arg(long, default_value = "menu")]
    pub command: String,
}