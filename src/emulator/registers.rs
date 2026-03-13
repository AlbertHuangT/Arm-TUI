use unicorn_engine::RegisterARM;

/// ARM 寄存器名称列表（按展示顺序）
pub const REG_NAMES: &[&str] = &[
    "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7",
    "r8", "r9", "r10", "fp", "ip", "sp", "lr", "pc",
];

/// 寄存器名 → Unicorn RegisterARM
pub fn reg_id(name: &str) -> RegisterARM {
    match name {
        "r0"  => RegisterARM::R0,
        "r1"  => RegisterARM::R1,
        "r2"  => RegisterARM::R2,
        "r3"  => RegisterARM::R3,
        "r4"  => RegisterARM::R4,
        "r5"  => RegisterARM::R5,
        "r6"  => RegisterARM::R6,
        "r7"  => RegisterARM::R7,
        "r8"  => RegisterARM::R8,
        "r9"  => RegisterARM::R9,
        "r10" => RegisterARM::R10,
        "fp"  => RegisterARM::R11,
        "ip"  => RegisterARM::R12,
        "sp"  => RegisterARM::R13,
        "lr"  => RegisterARM::R14,
        "pc"  => RegisterARM::R15,
        _     => RegisterARM::R0,
    }
}

/// CPSR 标志位解析
pub struct CpsrFlags {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,
}

impl CpsrFlags {
    pub fn from_cpsr(cpsr: u32) -> Self {
        Self {
            n: (cpsr >> 31) & 1 == 1,
            z: (cpsr >> 30) & 1 == 1,
            c: (cpsr >> 29) & 1 == 1,
            v: (cpsr >> 28) & 1 == 1,
        }
    }
}
