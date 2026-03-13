use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};
use crate::emulator::state::{EmulatorState, StackWord};

const SP_COLOR: Color = Color::Rgb(0, 255, 136);
const FP_COLOR: Color = Color::Rgb(255, 204, 0);
const FADE_IN_COLOR: Color = Color::Rgb(0, 200, 100);
const FADE_OUT_COLOR: Color = Color::Rgb(200, 60, 60);

fn lerp_color(from: Color, to: Color, t: f32) -> Color {
    // 简单线性插值（仅 Rgb 类型）
    if let (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) = (from, to) {
        let r = (r1 as f32 * (1.0 - t) + r2 as f32 * t) as u8;
        let g = (g1 as f32 * (1.0 - t) + g2 as f32 * t) as u8;
        let b = (b1 as f32 * (1.0 - t) + b2 as f32 * t) as u8;
        Color::Rgb(r, g, b)
    } else {
        from
    }
}

pub struct StackPanel<'a> {
    pub state: &'a EmulatorState,
}

impl<'a> Widget for StackPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let sp = self.state.sp();
        let fp = self.state.fp();

        // 表头
        let header = ListItem::new(Line::from(vec![
            Span::styled(
                format!("{:>10}  {:>8}  B3(MSB) B2      B1      B0(LSB)  {:>10}  标注",
                    "地址", "十六进制", "十进制"),
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
            ),
        ]));

        let mut items: Vec<ListItem> = vec![header];

        for word in &self.state.stack {
            let row = build_row(word, sp, fp);
            items.push(row);
        }

        let block = Block::default()
            .title(" 栈 (高地址 → 低地址) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        List::new(items).block(block).render(area, buf);
    }
}

fn build_row(word: &StackWord, sp: u32, fp: u32) -> ListItem<'static> {
    let val = word.value_u32();
    let [b0, b1, b2, b3] = word.bytes; // 小端：b0=LSB, b3=MSB

    // 地址行颜色
    let addr_color = if word.addr == sp {
        SP_COLOR
    } else if word.addr == fp {
        FP_COLOR
    } else if word.in_use {
        Color::White
    } else {
        Color::DarkGray
    };

    // 动画颜色混合
    let base_color = if word.fade_in > 0 {
        let t = 1.0 - word.fade_in as f32 / 8.0;
        lerp_color(FADE_IN_COLOR, addr_color, t)
    } else if word.fade_out > 0 {
        let t = word.fade_out as f32 / 8.0;
        lerp_color(Color::Rgb(40, 40, 40), FADE_OUT_COLOR, t)
    } else {
        addr_color
    };

    // 标注
    let annotation = if word.addr == sp {
        "← SP"
    } else if word.addr == fp {
        "← FP"
    } else if !word.in_use {
        if val == 0 { "[zero]" } else { "(uninit)" }
    } else {
        ""
    };

    let addr_style = Style::default().fg(base_color).add_modifier(
        if word.addr == sp || word.addr == fp { Modifier::BOLD } else { Modifier::empty() },
    );

    let byte_color = if word.in_use { Color::Gray } else { Color::DarkGray };
    let val_color  = if word.in_use { Color::White } else { Color::DarkGray };

    let spans = vec![
        Span::styled(format!("0x{:08X}  ", word.addr), addr_style),
        Span::styled(format!("0x{:08X}  ", val), Style::default().fg(val_color)),
        Span::styled(format!("0x{:02X}    ", b3), Style::default().fg(byte_color)),
        Span::styled(format!("0x{:02X}    ", b2), Style::default().fg(byte_color)),
        Span::styled(format!("0x{:02X}    ", b1), Style::default().fg(byte_color)),
        Span::styled(format!("0x{:02X}     ", b0), Style::default().fg(byte_color)),
        Span::styled(format!("{:>10}  ", val as i32), Style::default().fg(Color::DarkGray)),
        Span::styled(annotation.to_string(), Style::default().fg(base_color).add_modifier(Modifier::BOLD)),
    ];

    ListItem::new(Line::from(spans))
}
