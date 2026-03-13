# arm-tui

A terminal UI debugger for ARM v7 assembly, built in Rust.

Step through ARM32 programs instruction by instruction, watch registers change in real time, and visualize the stack with push/pop animations — all inside your terminal.

![ARM TUI demo](https://raw.githubusercontent.com/AlbertHuangT/Arm-TUI/main/docs/demo.gif)

## Features

- **Code panel** — source listing with current-line indicator (`►`) and breakpoints (`●`)
- **Register panel** — r0–r15 + CPSR with N/Z/C/V flags; changed values highlighted in green; `sp` in green, `fp` in yellow
- **Stack panel** — per-word byte-level display (B0 LSB … B3 MSB); push fades in, pop fades out; `← SP` / `← FP` annotations
- **Syscall emulation** — `sys_write` output shown in the OUTPUT panel; `sys_exit` terminates cleanly
- **Instruction whitelist** — only standard ARM32 instructions are permitted; illegal instructions halt execution with an error message
- **Breakpoints** — toggle on the current line, run continuously until hit

## Installation

### Via Homebrew (recommended)

```bash
brew tap AlbertHuangT/arm-tui
brew install arm-tui
```

That's it — no Rust, no CMake, no manual patching required.

### Build from source

Requirements: macOS, Rust toolchain (`rustup`), CMake and Unicorn via Homebrew.

```bash
brew install cmake unicorn
git clone https://github.com/AlbertHuangT/Arm-TUI.git
cd Arm-TUI
cargo install --locked --offline --path .
```

## Usage

```bash
arm-tui <file.s>
```

Example:

```bash
arm-tui examples/stack_demo.s
```

### Keyboard shortcuts

| Key | Action |
|-----|--------|
| `s` | Step — execute one instruction |
| `r` | Run — continue until next breakpoint or end |
| `R` (Shift+R) | Reset — restart from the beginning |
| `b` | Breakpoint — toggle on current line |
| `j` / `↓` | Scroll code panel down |
| `k` / `↑` | Scroll code panel up |
| `q` | Quit |

## Writing assembly

Programs must be ARM32 (not Thumb). Use the standard Linux ARM EABI calling convention.

Supported syscalls:

| r7 | Syscall | Description |
|----|---------|-------------|
| 4  | `sys_write` | Write to stdout (fd in r0, buf in r1, count in r2) |
| 1  | `sys_exit` | Exit emulation |

Example (`examples/hello.s`):

```asm
.global main
main:
    @ print "Hello!\n"
    mov r0, #1          @ fd = stdout
    ldr r1, =msg
    mov r2, #7
    mov r7, #4          @ sys_write
    svc #0

    mov r7, #1          @ sys_exit
    svc #0

.data
msg: .ascii "Hello!\n"
```

## Memory layout

| Region | Address | Size |
|--------|---------|------|
| Code   | `0x10000000` | 4 MB |
| Data   | `0x20000000` | 4 MB |
| Stack  | `0x8C000000`–`0x90000000` | 4 MB (grows down) |

SP is initialized to `0x90000000`.

## License

MIT
