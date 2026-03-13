use anyhow::Result;
use keystone_engine::{Arch, Keystone, Mode};
use std::collections::HashMap;

/// 汇编结果
pub struct AssembledProgram {
    /// 机器码字节流
    pub code: Vec<u8>,
    /// 地址 → 源码行号（0-indexed）的映射
    pub addr_to_line: HashMap<u32, usize>,
    /// 代码加载起始地址
    pub base_addr: u32,
}

pub struct Assembler {
    ks: Keystone,
}

impl Assembler {
    pub fn new() -> Result<Self> {
        let ks = Keystone::new(Arch::ARM, Mode::ARM)
            .map_err(|e| anyhow::anyhow!("Keystone 初始化失败: {:?}", e))?;
        Ok(Self { ks })
    }

    /// 将完整源码一次性汇编，并建立 addr→line 映射
    pub fn assemble(&self, source: &str) -> Result<AssembledProgram> {
        use crate::emulator::memory::CODE_BASE;

        // --- 第一遍：整体汇编获取机器码 ---
        let result = self
            .ks
            .asm(source.to_string(), CODE_BASE)
            .map_err(|e| anyhow::anyhow!("汇编失败: {:?}", e))?;

        let code = result.bytes;

        // --- 第二遍：逐行汇编建立映射 ---
        // 只对产生机器码的行（非空行、非纯指令标签、非伪指令如 .global/.syntax）建立映射
        let mut addr_to_line: HashMap<u32, usize> = HashMap::new();
        let mut cursor: u32 = CODE_BASE as u32;

        for (line_idx, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            // 跳过空行、注释行、以 '.' 开头的伪指令、以 '@' 开头的注释
            if trimmed.is_empty()
                || trimmed.starts_with('@')
                || trimmed.starts_with("//")
                || trimmed.starts_with(".syntax")
                || trimmed.starts_with(".arch")
                || trimmed.starts_with(".cpu")
                || trimmed.starts_with(".thumb")
                || trimmed.starts_with(".arm")
                || trimmed.starts_with(".global")
                || trimmed.starts_with(".globl")
                || trimmed.starts_with(".text")
                || trimmed.starts_with(".data")
                || trimmed.starts_with(".section")
                || trimmed.starts_with(".equ")
                || trimmed.starts_with(".set")
                || trimmed.starts_with(".align")
                || trimmed.starts_with(".word")
                || trimmed.starts_with(".byte")
                || trimmed.starts_with(".space")
                || trimmed.starts_with(".string")
                || trimmed.starts_with(".ascii")
                || trimmed.starts_with(".end")
            {
                continue;
            }

            // 纯标签行（如 "main:" 或 ".L1:"）不占机器码
            if trimmed.ends_with(':') && !trimmed.contains(' ') {
                continue;
            }

            // 尝试汇编单行（携带游标地址以正确处理 PC-relative）
            // 如果含有标签前缀，只取标签后面的部分
            let insn_part = if let Some(colon_pos) = trimmed.find(':') {
                let after = trimmed[colon_pos + 1..].trim();
                if after.is_empty() { continue; } else { after }
            } else {
                trimmed
            };

            match self.ks.asm(insn_part.to_string(), cursor as u64) {
                Ok(r) if !r.bytes.is_empty() => {
                    addr_to_line.insert(cursor, line_idx);
                    cursor += r.bytes.len() as u32;
                }
                _ => {
                    // 汇编失败（如数据伪指令）或空输出，跳过
                }
            }
        }

        Ok(AssembledProgram {
            code,
            addr_to_line,
            base_addr: CODE_BASE as u32,
        })
    }
}
