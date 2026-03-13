use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};
use crate::emulator::state::{EmulatorState, EmulatorStatus};

pub struct StatusBar<'a> {
    pub state: &'a EmulatorState,
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (status_text, status_color) = match &self.state.status {
            EmulatorStatus::Idle     => ("● 初始化", Color::DarkGray),
            EmulatorStatus::Paused   => ("‖ 已暂停", Color::Yellow),
            EmulatorStatus::Running  => ("▶ 运行中", Color::Green),
            EmulatorStatus::Finished => ("✓ 完成", Color::LightGreen),
            EmulatorStatus::Error    => ("✗ 错误", Color::LightRed),
        };

        let error_part = if let Some(e) = &self.state.error {
            format!("  错误: {}", e)
        } else {
            String::new()
        };

        let help = "  [s]单步  [r]运行  [b]断点  [R]重置  [q]退出";

        let line = Line::from(vec![
            Span::styled(
                format!(" {} ", status_text),
                Style::default().fg(status_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                error_part,
                Style::default().fg(Color::LightRed),
            ),
            Span::styled(
                help,
                Style::default().fg(Color::DarkGray),
            ),
        ]);

        line.render(area, buf);
    }
}
