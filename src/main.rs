mod cli;
mod emulator;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use emulator::assembler::Assembler;
use emulator::engine::Engine;
use emulator::state::EmulatorState;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let source = std::fs::read_to_string(&cli.file)
        .map_err(|e| anyhow::anyhow!("无法读取文件 {:?}: {}", cli.file, e))?;

    let assembler = Assembler::new()?;
    let assembled = assembler.assemble(&source)?;

    let file_name = cli
        .file
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let source_lines: Vec<String> = source.lines().map(|l| l.to_string()).collect();
    let state = EmulatorState::new(source_lines, file_name);
    let engine = Engine::new(assembled)?;

    ui::run(state, engine)?;
    Ok(())
}
