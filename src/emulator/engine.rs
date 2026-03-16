use anyhow::Result;
use std::collections::HashSet;
use unicorn_engine::{
    unicorn_const::{Arch, Mode, Prot},
    RegisterARM, Unicorn,
};

use crate::emulator::{
    assembler::AssembledProgram,
    memory::{CODE_BASE, CODE_SIZE, DATA_BASE, DATA_SIZE, STACK_BASE, STACK_SIZE, STACK_TOP, STACK_DISPLAY_WORDS},
    state::{EmulatorState, EmulatorStatus, StackWord},
    whitelist::default_whitelist,
};

struct HookData {
    whitelist: HashSet<u32>,
    error: Option<String>,
    output: String,
}

pub struct Engine {
    assembled: AssembledProgram,
}

impl Engine {
    pub fn new(assembled: AssembledProgram) -> Result<Self> {
        Ok(Self { assembled })
    }

    fn init_unicorn(&self) -> Result<Unicorn<HookData>> {
        let mut uc = Unicorn::new_with_data(
            Arch::ARM,
            Mode::ARM,
            HookData {
                whitelist: default_whitelist(),
                error: None,
                output: String::new(),
            },
        )
        .map_err(|e| anyhow::anyhow!("Unicorn 初始化失败: {:?}", e))?;

        uc.mem_map(CODE_BASE, CODE_SIZE, Prot::ALL)
            .map_err(|e| anyhow::anyhow!("映射代码段失败: {:?}", e))?;
        uc.mem_map(DATA_BASE, DATA_SIZE, Prot::ALL)
            .map_err(|e| anyhow::anyhow!("映射数据段失败: {:?}", e))?;
        uc.mem_map(STACK_BASE, STACK_SIZE, Prot::ALL)
            .map_err(|e| anyhow::anyhow!("映射栈段失败: {:?}", e))?;

        uc.mem_write(CODE_BASE, &self.assembled.code)
            .map_err(|e| anyhow::anyhow!("写入代码段失败: {:?}", e))?;

        uc.reg_write(RegisterARM::SP as i32, STACK_TOP)
            .map_err(|e| anyhow::anyhow!("设置 SP 失败: {:?}", e))?;
        uc.reg_write(RegisterARM::PC as i32, CODE_BASE)
            .map_err(|e| anyhow::anyhow!("设置 PC 失败: {:?}", e))?;

        Ok(uc)
    }

    fn restore_unicorn(&self, state: &EmulatorState) -> Result<Unicorn<HookData>> {
        let mut uc = self.init_unicorn()?;

        let reg_ids = [
            RegisterARM::R0, RegisterARM::R1, RegisterARM::R2, RegisterARM::R3,
            RegisterARM::R4, RegisterARM::R5, RegisterARM::R6, RegisterARM::R7,
            RegisterARM::R8, RegisterARM::R9, RegisterARM::R10, RegisterARM::R11,
            RegisterARM::R12, RegisterARM::R13, RegisterARM::R14, RegisterARM::R15,
        ];
        for (i, reg) in reg_ids.iter().enumerate() {
            uc.reg_write(*reg as i32, state.regs[i] as u64)
                .map_err(|e| anyhow::anyhow!("恢复寄存器 r{i} 失败: {:?}", e))?;
        }
        uc.reg_write(RegisterARM::CPSR as i32, state.cpsr as u64)
            .map_err(|e| anyhow::anyhow!("恢复 CPSR 失败: {:?}", e))?;

        for word in &state.stack {
            if word.in_use {
                uc.mem_write(word.addr as u64, &word.bytes).ok();
            }
        }
        Ok(uc)
    }

    fn add_hooks(uc: &mut Unicorn<HookData>) -> Result<()> {
        // 白名单 hook
        uc.add_code_hook(CODE_BASE, CODE_BASE + CODE_SIZE, |uc, addr, _size| {
            let bytes = match uc.mem_read_as_vec(addr, 4) {
                Ok(b) => b,
                Err(_) => return,
            };
            // 在作用域内提取所需信息，避免生命周期问题
            let insn_info: Option<(u32, String)> = {
                use capstone::prelude::*;
                match Capstone::new()
                    .arm()
                    .mode(capstone::arch::arm::ArchMode::Arm)
                    .build()
                {
                    Ok(cs) => {
                        cs.disasm_count(&bytes, addr, 1).ok().and_then(|insns| {
                            insns.iter().next().map(|i| {
                                (i.id().0, i.mnemonic().unwrap_or("?").to_string())
                            })
                        })
                    }
                    Err(_) => None,
                }
            };
            if let Some((id, mnem)) = insn_info {
                let wl = &uc.get_data().whitelist;
                if !wl.contains(&id) {
                    uc.get_data_mut().error =
                        Some(format!("指令 '{}' 不在允许列表中，执行已终止", mnem));
                    let _ = uc.emu_stop();
                }
            }
        })
        .map_err(|e| anyhow::anyhow!("注册 CODE hook 失败: {:?}", e))?;

        // SWI/SVC hook
        uc.add_intr_hook(|uc, _intno| {
            let r7 = uc.reg_read(RegisterARM::R7 as i32).unwrap_or(0) as u32;
            match r7 {
                4 => {
                    let buf = uc.reg_read(RegisterARM::R1 as i32).unwrap_or(0);
                    let count = uc.reg_read(RegisterARM::R2 as i32).unwrap_or(0) as usize;
                    if let Ok(bytes) = uc.mem_read_as_vec(buf, count) {
                        if let Ok(s) = std::str::from_utf8(&bytes) {
                            uc.get_data_mut().output.push_str(s);
                        }
                    }
                }
                1 | 248 => {
                    let _ = uc.emu_stop();
                }
                _ => {}
            }
        })
        .map_err(|e| anyhow::anyhow!("注册 INTR hook 失败: {:?}", e))?;

        // 非法内存访问 hook，防止 Unicorn native 崩溃
        uc.add_mem_hook(
            unicorn_engine::unicorn_const::HookType::MEM_INVALID,
            0, u64::MAX,
            |uc, _mem_type, addr, _size, _value| {
                uc.get_data_mut().error = Some(format!("访问未映射内存: 0x{:08x}", addr));
                let _ = uc.emu_stop();
                false
            },
        )
        .map_err(|e| anyhow::anyhow!("注册 MEM_INVALID hook 失败: {:?}", e))?;

        Ok(())
    }

    pub fn step(&self, state: &mut EmulatorState) -> Result<()> {
        if state.status == EmulatorStatus::Finished || state.status == EmulatorStatus::Error {
            return Ok(());
        }

        let pc = state.pc();
        let code_start = self.assembled.base_addr as u64;
        let code_end = code_start + self.assembled.code.len() as u64;
        if (pc as u64) < code_start || (pc as u64) >= code_end {
            state.status = EmulatorStatus::Finished;
            return Ok(());
        }

        state.prev_regs = state.regs;
        state.prev_cpsr = state.cpsr;
        state.prev_stack = state.stack.clone();

        let mut uc = self.restore_unicorn(state)?;
        Self::add_hooks(&mut uc)?;

        let _ = uc.emu_start(pc as u64, CODE_BASE + CODE_SIZE, 0, 1);

        if let Some(e) = uc.get_data().error.clone() {
            state.error = Some(e);
            state.status = EmulatorStatus::Error;
            return Ok(());
        }

        // 回读寄存器
        let reg_ids = [
            RegisterARM::R0, RegisterARM::R1, RegisterARM::R2, RegisterARM::R3,
            RegisterARM::R4, RegisterARM::R5, RegisterARM::R6, RegisterARM::R7,
            RegisterARM::R8, RegisterARM::R9, RegisterARM::R10, RegisterARM::R11,
            RegisterARM::R12, RegisterARM::R13, RegisterARM::R14, RegisterARM::R15,
        ];
        for (i, reg) in reg_ids.iter().enumerate() {
            state.regs[i] = uc.reg_read(*reg as i32).unwrap_or(0) as u32;
        }
        state.cpsr = uc.reg_read(RegisterARM::CPSR as i32).unwrap_or(0) as u32;

        let output = uc.get_data().output.clone();
        if !output.is_empty() {
            state.output_buffer.push_str(&output);
        }

        let new_pc = state.regs[15];
        state.current_line = self.assembled.addr_to_line.get(&new_pc).copied();

        self.update_stack(state, &uc);

        if state.status == EmulatorStatus::Idle {
            state.status = EmulatorStatus::Paused;
        }

        Ok(())
    }

    fn update_stack(&self, state: &mut EmulatorState, uc: &Unicorn<HookData>) {
        let sp = state.sp();
        let top = STACK_TOP as u32;
        let display_bottom = top.saturating_sub((STACK_DISPLAY_WORDS as u32) * 4);

        let mut new_stack: Vec<StackWord> = Vec::new();
        let mut addr = top - 4;

        loop {
            if addr < display_bottom {
                break;
            }
            let bytes = uc
                .mem_read_as_vec(addr as u64, 4)
                .map(|v| [v[0], v[1], v[2], v[3]])
                .unwrap_or([0u8; 4]);

            let in_use = addr >= sp;

            let prev = state.prev_stack.iter().find(|w| w.addr == addr);
            let (fade_in, fade_out) = match prev {
                None if in_use => (8, 0),
                Some(p) if p.in_use && !in_use => (0, 8),
                Some(p) => (p.fade_in.saturating_sub(1), p.fade_out.saturating_sub(1)),
                None => (0, 0),
            };

            new_stack.push(StackWord { addr, bytes, in_use, fade_in, fade_out });
            if addr == 0 { break; }
            addr -= 4;
        }

        state.stack = new_stack;
    }

    pub fn reset(&self, state: &mut EmulatorState) {
        state.regs = [0u32; 16];
        state.regs[13] = STACK_TOP as u32;
        state.regs[15] = self.assembled.base_addr;
        state.prev_regs = state.regs;
        state.cpsr = 0;
        state.prev_cpsr = 0;
        state.current_line = self
            .assembled
            .addr_to_line
            .get(&self.assembled.base_addr)
            .copied();
        state.stack = Vec::new();
        state.prev_stack = Vec::new();
        state.output_buffer = String::new();
        state.status = EmulatorStatus::Paused;
        state.error = None;
    }
}
