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

## Requirements

- macOS (Apple Silicon or Intel)
- Rust toolchain (`rustup`)
- CMake, Unicorn, Keystone (installed via Homebrew)

## Installation

### 1. Install system dependencies

```bash
brew install cmake unicorn keystone
```

### 2. Patch keystone-engine for modern CMake (one-time fix)

The `keystone-engine` crate bundles CMakeLists.txt files that are incompatible with CMake 4.x. Run this after the first `cargo build` attempt fails, or preemptively:

```bash
BASE=$(ls -d ~/.cargo/registry/src/*/keystone-engine-0.1.0/keystone 2>/dev/null | head -1)
for f in "$BASE/CMakeLists.txt" "$BASE/llvm/CMakeLists.txt" \
          "$BASE/samples/CMakeLists.txt" "$BASE/kstool/CMakeLists.txt"; do
  [ -f "$f" ] || continue
  sed -i '' 's/cmake_minimum_required(VERSION [0-9.]*)/cmake_minimum_required(VERSION 3.10)/g' "$f"
  sed -i '' '/cmake_policy(SET CMP0051/d' "$f"
done
```

### 3. Build and install

```bash
git clone https://github.com/AlbertHuangT/Arm-TUI.git
cd Arm-TUI
cargo install --path .
```

Or just build locally:

```bash
cargo build --release
```

The binary will be at `target/release/arm-tui`.

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
