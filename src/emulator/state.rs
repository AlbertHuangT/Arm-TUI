use std::collections::HashSet;

/// 栈中一个 word 的快照（4 字节，小端序）
#[derive(Clone, Debug)]
pub struct StackWord {
    /// 该 word 的地址（4 字节对齐）
    pub addr: u32,
    /// 4 个字节 [addr+0 .. addr+3]，小端序存储
    pub bytes: [u8; 4],
    /// true = SP 之上（已分配），false = SP 之下（未初始化）
    pub in_use: bool,
    /// push 时淡入帧数（0 = 无动画）
    pub fade_in: u8,
    /// pop 时淡出帧数（0 = 无动画）
    pub fade_out: u8,
}

impl StackWord {
    pub fn value_u32(&self) -> u32 {
        u32::from_le_bytes(self.bytes)
    }
}

/// 模拟器运行状态
#[derive(Clone, Debug, PartialEq)]
pub enum EmulatorStatus {
    Idle,
    Paused,
    Running,
    Finished,
    Error,
}

/// 模拟器完整状态快照（每一步更新后传给 UI）
#[derive(Clone, Debug)]
pub struct EmulatorState {
    // ── 源文件信息 ──────────────────────────────
    /// 源码各行文本
    pub source_lines: Vec<String>,
    /// 文件名（标题栏显示）
    pub file_name: String,

    // ── 寄存器 ──────────────────────────────────
    /// 当前寄存器值 [r0..r15]，下标 = 寄存器编号
    pub regs: [u32; 16],
    /// 上一步寄存器值（用于高亮 diff）
    pub prev_regs: [u32; 16],
    /// 当前 CPSR
    pub cpsr: u32,
    /// 上一步 CPSR
    pub prev_cpsr: u32,

    // ── 代码定位 ─────────────────────────────────
    /// 当前 PC 对应的源码行号（0-indexed），None = 未知
    pub current_line: Option<usize>,

    // ── 栈 ───────────────────────────────────────
    /// 栈快照：高地址 → 低地址排列
    pub stack: Vec<StackWord>,
    /// 上一步栈（用于检测 push/pop 动画）
    pub prev_stack: Vec<StackWord>,

    // ── 断点 ─────────────────────────────────────
    /// 断点行号集合（0-indexed）
    pub breakpoints: HashSet<usize>,

    // ── 输出 & 状态 ───────────────────────────────
    /// SWI write 输出缓冲
    pub output_buffer: String,
    pub status: EmulatorStatus,
    pub error: Option<String>,
}

impl EmulatorState {
    pub fn new(source_lines: Vec<String>, file_name: String) -> Self {
        Self {
            source_lines,
            file_name,
            regs: [0u32; 16],
            prev_regs: [0u32; 16],
            cpsr: 0,
            prev_cpsr: 0,
            current_line: None,
            stack: Vec::new(),
            prev_stack: Vec::new(),
            breakpoints: HashSet::new(),
            output_buffer: String::new(),
            status: EmulatorStatus::Idle,
            error: None,
        }
    }

    /// SP 值（寄存器 13）
    pub fn sp(&self) -> u32 { self.regs[13] }
    /// FP 值（寄存器 11）
    pub fn fp(&self) -> u32 { self.regs[11] }
    /// PC 值（寄存器 15）
    pub fn pc(&self) -> u32 { self.regs[15] }

    /// 切换断点：有则删，无则加
    pub fn toggle_breakpoint(&mut self, line: usize) {
        if self.breakpoints.contains(&line) {
            self.breakpoints.remove(&line);
        } else {
            self.breakpoints.insert(line);
        }
    }

    /// 推进动画帧计数器（在 Tick 事件时调用）
    pub fn tick_animation(&mut self) {
        for word in &mut self.stack {
            if word.fade_in > 0 {
                word.fade_in -= 1;
            }
            if word.fade_out > 0 {
                word.fade_out -= 1;
            }
        }
        // 删除已淡出完毕的条目
        self.stack.retain(|w| w.fade_out > 0 || w.in_use || w.fade_in > 0);
    }
}
