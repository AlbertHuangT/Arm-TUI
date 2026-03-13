use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "arm-tui", about = "ARM v7 汇编交互式 TUI 调试器")]
pub struct Cli {
    /// ARM 汇编源文件路径 (.s)
    pub file: PathBuf,
}
