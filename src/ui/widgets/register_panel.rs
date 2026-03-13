use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};
use crate::emulator::{registers::{CpsrFlags, REG_NAMES}, state::EmulatorState};

const CHANGED_COLOR: Color = Color::LightGreen;
const SP_COLOR: Color = Color::Rgb(0, 255, 136);
const FP_COLOR: Color = Color::Rgb(255, 204, 0);

pub struct RegisterPanel<'a> {
    pub state: &'a EmulatorState,
}

impl<'a> Widget for RegisterPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut items: Vec<ListItem> = REG_NAMES
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let val = self.state.regs[i];
                let prev = self.state.prev_regs[i];
                let changed = val != prev;

                let name_color = match *name {
                    "sp" => SP_COLOR,
                    "fp" => FP_COLOR,
                    _ if changed => CHANGED_COLOR,
                    _ => Color::White,
                };

                let name_span = Span::styled(
                    format!("{:>3}: ", name),
                    Style::default().fg(name_color).add_modifier(if changed { Modifier::BOLD } else { Modifier::empty() }),
                );
                let val_span = Span::styled(
                    format!("0x{:08X}", val),
                    Style::default().fg(if changed { CHANGED_COLOR } else { Color::Gray }),
                );
                let dec_span = Span::styled(
                    format!("  {:>10}", val as i32),
                    Style::default().fg(Color::DarkGray),
                );

                ListItem::new(Line::from(vec![name_span, val_span, dec_span]))
            })
            .collect();

        // CPSR 行
        let flags = CpsrFlags::from_cpsr(self.state.cpsr);
        let cpsr_changed = self.state.cpsr != self.state.prev_cpsr;
        let flag_str = format!(
            " N:{} Z:{} C:{} V:{}",
            if flags.n { '1' } else { '0' },
            if flags.z { '1' } else { '0' },
            if flags.c { '1' } else { '0' },
            if flags.v { '1' } else { '0' },
        );
        let cpsr_span = Span::styled(
            format!("CPSR: 0x{:08X}{}", self.state.cpsr, flag_str),
            Style::default()
                .fg(if cpsr_changed { CHANGED_COLOR } else { Color::Gray })
                .add_modifier(if cpsr_changed { Modifier::BOLD } else { Modifier::empty() }),
        );
        items.push(ListItem::new(Line::from(vec![cpsr_span])));

        let block = Block::default()
            .title(" 寄存器 ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        List::new(items).block(block).render(area, buf);
    }
}
