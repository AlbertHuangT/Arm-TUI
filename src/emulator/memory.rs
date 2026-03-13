/// 代码段起始地址
pub const CODE_BASE: u64 = 0x1000_0000;
/// 代码段大小 4MB
pub const CODE_SIZE: u64 = 0x40_0000;

/// 数据段起始地址
pub const DATA_BASE: u64 = 0x2000_0000;
/// 数据段大小 4MB
pub const DATA_SIZE: u64 = 0x40_0000;

/// 栈段起始（低地址端）
pub const STACK_BASE: u64 = 0x8C00_0000;
/// 栈段大小 4MB
pub const STACK_SIZE: u64 = 0x40_0000;
/// SP 初始值（栈顶，栈向下增长）
pub const STACK_TOP: u64 = STACK_BASE + STACK_SIZE; // 0x9000_0000

/// 栈面板展示范围：SP 上方 N 个 word
pub const STACK_DISPLAY_WORDS: usize = 32;
