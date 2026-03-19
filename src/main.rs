use ccstatusline::core::statusline::StatusLine;
use ccstatusline::model::InputData;
use clap::Parser;
use std::io::{self, Write};

use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    debug: bool,
}

/// 展开 ~ 为用户目录（Windows 和 Unix 都支持）
fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();

    // 检查是否以 ~ 开头
    let path_str = path.to_string_lossy();
    if !path_str.starts_with('~') {
        return path.to_path_buf();
    }

    // 获取用户目录
    let home = dirs::home_dir()
        .or_else(|| std::env::var("USERPROFILE").ok().map(PathBuf::from))
        .or_else(|| std::env::var("HOME").ok().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));

    // 替换 ~ 为 home
    if path_str == "~" {
        home
    } else {
        // ~/foo/bar → C:\Users\name\foo\bar
        let rest = &path_str[2..]; // 跳过 "~/"
        home.join(rest)
    }
}

/// 获取默认调试路径（支持 ~）
fn default_debug_path() -> PathBuf {
    expand_tilde("~/.claude/ccline/statusline.json")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // 根据 --debug 标志选择输入源
    let config = if cli.debug {
        let expanded = expand_tilde(&default_debug_path());

        eprintln!("🔧 Debug: reading from {}", expanded.display());

        let content = tokio::fs::read_to_string(&expanded)
            .await
            .map_err(|e| format!("Cannot read {}: {}", expanded.display(), e))?;

        serde_json::from_str(&content).map_err(|e| format!("Invalid JSON: {}", e))?
    } else {
        // 正常模式：从 stdin 读取
        let stdin = io::stdin();
        serde_json::from_reader::<_, InputData>(stdin.lock())?
    };
    let out_path = &default_debug_path();

    // 确保父目录存在
    if let Some(parent) = out_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let json = serde_json::to_vec_pretty(&config)?;
    tokio::fs::write(&out_path, json).await?;
    let stdout = io::stdout();
    writeln!(&stdout, "{}", StatusLine::generate(&config).await)?;

    Ok(())
}
