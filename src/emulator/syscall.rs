use unicorn_engine::Unicorn;

/// 模拟 Linux ARM EABI 系统调用
/// r7 = 系统调用号，参数依次 r0..r5
pub fn handle_swi<D>(uc: &mut Unicorn<D>, output: &mut String) {
    let r7 = uc.reg_read(unicorn_engine::RegisterARM::R7 as i32).unwrap_or(0) as u32;
    match r7 {
        // sys_write(fd, buf, count)
        4 => {
            let _fd  = uc.reg_read(unicorn_engine::RegisterARM::R0 as i32).unwrap_or(0) as u32;
            let buf  = uc.reg_read(unicorn_engine::RegisterARM::R1 as i32).unwrap_or(0) as u64;
            let count= uc.reg_read(unicorn_engine::RegisterARM::R2 as i32).unwrap_or(0) as usize;
            if let Ok(bytes) = uc.mem_read_as_vec(buf, count) {
                if let Ok(s) = std::str::from_utf8(&bytes) {
                    output.push_str(s);
                }
            }
        }
        // sys_exit / sys_exit_group
        1 | 248 => {
            let _ = uc.emu_stop();
        }
        _ => {}
    }
}
