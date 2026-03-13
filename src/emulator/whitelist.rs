use std::collections::HashSet;

/// 默认允许指令集（Capstone ARM InsnId 值）
/// 注意：Capstone 将 S 后缀（如 ADDS）和基础指令（ADD）共享同一 InsnId，
/// 通过 detail 的 update_flags 字段区分，故白名单只需包含基础 mnemonic 对应的 id。
pub fn default_whitelist() -> HashSet<u32> {
    use capstone::arch::arm::ArmInsn::*;
    let ids: &[capstone::arch::arm::ArmInsn] = &[
        // 数据处理
        ARM_INS_MOV, ARM_INS_MOVW, ARM_INS_MOVT, ARM_INS_MVN,
        ARM_INS_ADD, ARM_INS_ADC,
        ARM_INS_SUB, ARM_INS_SBC,
        ARM_INS_RSB, ARM_INS_RSC,
        ARM_INS_MUL, ARM_INS_MLA, ARM_INS_UMULL, ARM_INS_UMLAL,
        ARM_INS_SMULL, ARM_INS_SMLAL,
        // 逻辑
        ARM_INS_AND, ARM_INS_ORR, ARM_INS_EOR, ARM_INS_BIC,
        // 移位
        ARM_INS_LSL, ARM_INS_LSR, ARM_INS_ASR, ARM_INS_ROR, ARM_INS_RRX,
        // 比较/测试
        ARM_INS_CMP, ARM_INS_CMN, ARM_INS_TST, ARM_INS_TEQ,
        // 内存
        ARM_INS_LDR, ARM_INS_LDRB, ARM_INS_LDRH, ARM_INS_LDRSB, ARM_INS_LDRSH,
        ARM_INS_STR, ARM_INS_STRB, ARM_INS_STRH,
        ARM_INS_LDM, ARM_INS_LDMDA, ARM_INS_LDMDB, ARM_INS_LDMIB,
        ARM_INS_STM, ARM_INS_STMDA, ARM_INS_STMDB, ARM_INS_STMIB,
        ARM_INS_PUSH, ARM_INS_POP,
        // 分支
        ARM_INS_B, ARM_INS_BL, ARM_INS_BX, ARM_INS_BLX,
        // 系统调用
        ARM_INS_SVC,
        // NOP
        ARM_INS_NOP,
    ];
    ids.iter().map(|i| *i as u32).collect()
}
