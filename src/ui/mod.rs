pub mod app;
pub mod event;
pub mod widgets;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use crate::emulator::{engine::Engine, state::EmulatorState};
use app::App;
use event::next_event;

/// 启动 TUI 主循环
pub fn run(state: EmulatorState, engine: Engine) -> Result<()> {
    // 初始化终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(state, engine);

    // 主循环
    loop {
        terminal.draw(|f| app.draw(f))?;

        let ev = next_event()?;
        app.handle_event(ev)?;

        if app.should_quit {
            break;
        }
    }

    // 还原终端
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // 若有输出，打印到标准输出
    if !app.state.output_buffer.is_empty() {
        println!("{}", app.state.output_buffer);
    }

    Ok(())
}
