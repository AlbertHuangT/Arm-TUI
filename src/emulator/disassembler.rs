use anyhow::Result;
use capstone::prelude::*;

pub struct Disassembler {
    cs: Capstone,
}

impl Disassembler {
    pub fn new() -> Result<Self> {
        let cs = Capstone::new()
            .arm()
            .mode(arch::arm::ArchMode::Arm)
            .detail(true)
            .build()
            .map_err(|e| anyhow::anyhow!("Capstone 初始化失败: {:?}", e))?;
        Ok(Self { cs })
    }

    /// 反汇编单条指令，返回 (助记符, 操作数) 字符串
    pub fn disasm_one(&self, bytes: &[u8], addr: u64) -> Option<(String, String)> {
        let insns = self.cs.disasm_count(bytes, addr, 1).ok()?;
        let insn = insns.first()?;
        Some((
            insn.mnemonic().unwrap_or("?").to_string(),
            insn.op_str().unwrap_or("").to_string(),
        ))
    }

    /// 返回指令的 Capstone InsnId（u32），用于白名单检查
    pub fn insn_id(&self, bytes: &[u8], addr: u64) -> Option<u32> {
        let insns = self.cs.disasm_count(bytes, addr, 1).ok()?;
        Some(insns.first()?.id().0)
    }

    /// 暴露底层 Capstone 供 hook 使用
    pub fn capstone(&self) -> &Capstone {
        &self.cs
    }
}
