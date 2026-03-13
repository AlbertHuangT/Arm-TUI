use crossterm::event::{self, Event as CEvent, KeyEvent};
use std::time::Duration;

/// TUI 事件
pub enum Event {
    /// 键盘输入
    Key(KeyEvent),
    /// 定时 tick（50ms），用于推进动画
    Tick,
}

/// 从 crossterm 读取一个事件（阻塞最多 50ms）
pub fn next_event() -> anyhow::Result<Event> {
    if event::poll(Duration::from_millis(50))? {
        if let CEvent::Key(key) = event::read()? {
            return Ok(Event::Key(key));
        }
    }
    Ok(Event::Tick)
}
