use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, StatefulWidget, Widget},
};
use crate::emulator::state::EmulatorState;

const CURRENT_BG: Color = Color::Rgb(180, 90, 0);
const BP_COLOR: Color = Color::LightRed;

pub struct CodePanel<'a> {
    pub state: &'a EmulatorState,
}

pub struct CodePanelState {
    pub scroll: usize,
}

impl Default for CodePanelState {
    fn default() -> Self { Self { scroll: 0 } }
}

impl CodePanelState {
    /// 确保当前行可见
    pub fn ensure_visible(&mut self, current_line: Option<usize>, height: usize) {
        if let Some(line) = current_line {
            if line < self.scroll {
                self.scroll = line;
            } else if line >= self.scroll + height.saturating_sub(2) {
                self.scroll = line.saturating_sub(height.saturating_sub(3));
            }
        }
    }
}

impl<'a> StatefulWidget for CodePanel<'a> {
    type State = CodePanelState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let height = area.height as usize;
        state.ensure_visible(self.state.current_line, height);

        let items: Vec<ListItem> = self
            .state
            .source_lines
            .iter()
            .enumerate()
            .skip(state.scroll)
            .take(height.saturating_sub(2))
            .map(|(i, line)| {
                let is_current = Some(i) == self.state.current_line;
                let is_bp = self.state.breakpoints.contains(&i);

                let line_num = Span::styled(
                    format!("{:4} ", i + 1),
                    Style::default().fg(if is_current { Color::White } else { Color::DarkGray }),
                );
                let bp_marker = Span::styled(
                    if is_bp { "● " } else { "  " },
                    Style::default().fg(BP_COLOR),
                );
                let code = Span::raw(line.as_str());

                let style = if is_current {
                    Style::default().bg(CURRENT_BG).fg(Color::White).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(Line::from(vec![line_num, bp_marker, code])).style(style)
            })
            .collect();

        let title = format!(" {} ", self.state.file_name);
        let block = Block::default()
            .title(title.as_str())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        Widget::render(List::new(items).block(block), area, buf);
    }
}
