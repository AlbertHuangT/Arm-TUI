use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::emulator::{
    engine::Engine,
    state::{EmulatorState, EmulatorStatus},
};
use super::{
    event::Event,
    widgets::{
        code_panel::{CodePanel, CodePanelState},
        register_panel::RegisterPanel,
        stack_panel::StackPanel,
        status_bar::StatusBar,
    },
};

pub struct App {
    pub state: EmulatorState,
    pub engine: Engine,
    pub code_panel_state: CodePanelState,
    pub should_quit: bool,
}

impl App {
    pub fn new(state: EmulatorState, engine: Engine) -> Self {
        let mut app = Self {
            state,
            engine,
            code_panel_state: CodePanelState::default(),
            should_quit: false,
        };
        // 初始化：重置到第一条指令
        app.engine.reset(&mut app.state);
        app
    }

    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Tick => {
                self.state.tick_animation();
            }
            Event::Key(key) => match key.code {
                // 退出
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.should_quit = true;
                }
                // 单步
                KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::F(8) => {
                    if matches!(
                        self.state.status,
                        EmulatorStatus::Paused | EmulatorStatus::Idle
                    ) {
                        self.engine.step(&mut self.state)?;
                    }
                }
                // 运行到下一个断点
                KeyCode::Char('r') | KeyCode::Char('R') if key.modifiers == KeyModifiers::NONE => {
                    if matches!(self.state.status, EmulatorStatus::Paused) {
                        loop {
                            self.engine.step(&mut self.state)?;
                            if self.state.status != EmulatorStatus::Paused {
                                break;
                            }
                            if let Some(line) = self.state.current_line {
                                if self.state.breakpoints.contains(&line) {
                                    break;
                                }
                            }
                        }
                    }
                }
                // 重置
                KeyCode::Char('R') => {
                    self.engine.reset(&mut self.state);
                }
                // 切换断点（当前行）
                KeyCode::Char('b') | KeyCode::Char('B') => {
                    if let Some(line) = self.state.current_line {
                        self.state.toggle_breakpoint(line);
                    }
                }
                // 代码面板滚动
                KeyCode::Up | KeyCode::Char('k') => {
                    self.code_panel_state.scroll =
                        self.code_panel_state.scroll.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.code_panel_state.scroll += 1;
                }
                _ => {}
            },
        }
        Ok(())
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let size = f.area();

        // 顶层布局：主内容区 + 状态栏
        let root = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(size);

        // 主内容区：左 (代码) + 右 (寄存器 + 栈)
        let main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(root[0]);

        // 右侧：寄存器 (上) + 栈 (下)
        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(20), Constraint::Min(1)])
            .split(main[1]);

        // 渲染各面板
        let code_widget = CodePanel { state: &self.state };
        f.render_stateful_widget(code_widget, main[0], &mut self.code_panel_state);

        f.render_widget(RegisterPanel { state: &self.state }, right[0]);
        f.render_widget(StackPanel { state: &self.state }, right[1]);
        f.render_widget(StatusBar { state: &self.state }, root[1]);
    }
}
