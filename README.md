# STM32F429ZI — Bare-Metal Rust

Bare-metal Rust project for the **STM32F429I-DISC1** discovery board (STM32F429ZIT6 chip).
The firmware blinks the two on-board LEDs (PG13 green, PG14 red) using a TIM2 interrupt at 1 Hz.

---

## Hardware

| Property        | Value                                    |
|-----------------|------------------------------------------|
| MCU             | STM32F429ZIT6                            |
| Core            | ARM Cortex-M4F (with FPU)               |
| Flash           | 2048 KB at `0x0800_0000`                |
| RAM             | 192 KB at `0x2000_0000`                 |
| CCMRAM          | 64 KB at `0x1000_0000`                  |
| HSE crystal     | 8 MHz                                    |
| System clock    | 180 MHz                                  |
| Programmer      | ST-Link v2.1 (built into the board)     |
| Green LED       | PG13                                     |
| Red LED         | PG14                                     |

---

## Prerequisites

All steps assume **macOS** with [Homebrew](https://brew.sh) installed.

### 1 — Install Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen prompts. After installation, reload your shell:

```sh
source "$HOME/.cargo/env"
```

### 2 — Add the Cortex-M4F compilation target

The STM32F429ZI has a Cortex-M4F core (with FPU), so we need the `hf` (hard-float) variant:

```sh
rustup target add thumbv7em-none-eabihf
```

### 3 — Install OpenOCD

OpenOCD is the on-chip debugger that speaks to the ST-Link probe on the board:

```sh
brew install openocd
```

Verify: `openocd --version` should print `Open On-Chip Debugger 0.12.x` or newer.

### 4 — Install GDB

The Homebrew `gdb` package supports ARM targets on macOS:

```sh
brew install gdb
```

Verify: `gdb --version` should print `GNU gdb`.

> **Note:** `gdb-multiarch` does not exist on macOS. The `.cargo/config.toml` in this
> project already points to `gdb`.

### 5 — Install `cargo-binutils` (optional but useful)

Provides `cargo size`, `cargo objdump`, and `cargo nm` for inspecting the binary:

```sh
cargo install cargo-binutils
rustup component add llvm-tools
```

---

## Project structure

```
.
├── .cargo/
│   └── config.toml       # Build target + GDB runner
├── src/
│   └── main.rs           # Application entry point
├── memory.x              # Linker script — Flash/RAM regions for STM32F429ZI
├── build.rs              # Passes memory.x to the linker
├── openocd.cfg           # OpenOCD board config (ST-Link + STM32F4x target)
├── openocd.gdb           # GDB init script — connects, loads firmware, sets breakpoints
├── Makefile              # Convenience targets: build / flash / debug
└── Cargo.toml            # Crate dependencies
```

### Key files explained

**`memory.x`** — tells the linker where Flash and RAM live on this chip:

```
FLASH  : ORIGIN = 0x08000000, LENGTH = 2048K
RAM    : ORIGIN = 0x20000000, LENGTH = 192K
CCMRAM : ORIGIN = 0x10000000, LENGTH = 64K
```

**`.cargo/config.toml`** — sets the default build target and the GDB runner used by `cargo run`:

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "gdb -q -x openocd.gdb"

[build]
target = "thumbv7em-none-eabihf"
```

**`openocd.cfg`** — two lines that select the ST-Link interface and the STM32F4 target:

```tcl
source [find interface/stlink.cfg]
source [find target/stm32f4x.cfg]
```

**`openocd.gdb`** — run automatically by GDB on startup: connects to OpenOCD on port 3333,
loads the firmware onto Flash, sets breakpoints at `main`, `HardFault`, and `DefaultHandler`,
then steps one instruction so the breakpoint at `main` is reachable.

---

## Building

```sh
cargo build
```

The compiled ELF lands at `target/thumbv7em-none-eabihf/debug/stm32f429i-disc1`.

For an optimised release build:

```sh
cargo build --release
```

---

## Flashing

Connect the board via USB. The ST-Link port is the micro-USB connector closest to the
reset button. Then run:

```sh
make flash
```

This calls OpenOCD to program the ELF file, verify it, and reset the board. The LEDs
should start alternating immediately.

---

## Debugging

The `debug` Make target does everything in one shot:

```sh
make debug
```

What it does, step by step:

1. `cargo clean` — removes all previous build artefacts for a clean state.
2. `cargo build` — compiles the firmware.
3. Starts `openocd` in the background, connecting to the board via ST-Link.
4. Launches `gdb`, which runs `openocd.gdb` automatically:
   - Connects to OpenOCD on `localhost:3333`.
   - Flashes the firmware (`load`).
   - Sets breakpoints at `main`, `HardFault`, and `DefaultHandler`.
   - Halts at the first instruction.
5. GDB stops at `main` — you are now in an interactive debug session.
6. When you quit GDB (type `quit` or press `Ctrl-D`), OpenOCD is killed automatically.

### Useful GDB commands during a session

| Command            | Effect                                         |
|--------------------|------------------------------------------------|
| `c` or `continue`  | Resume execution                               |
| `n` or `next`      | Step over the current line                     |
| `s` or `step`      | Step into a function call                      |
| `p <expr>`         | Print a variable or expression                 |
| `info registers`   | Show all CPU registers                         |
| `bt`               | Print the call stack (backtrace)               |
| `break <fn>`       | Set a breakpoint at function `<fn>`            |
| `monitor reset halt` | Reset and halt the MCU via OpenOCD           |
| `quit`             | Exit GDB (also kills OpenOCD)                  |

---

## Makefile targets

| Target       | Description                                              |
|--------------|----------------------------------------------------------|
| `make`       | Build the project (alias for `make build`)              |
| `make build` | Compile the firmware                                     |
| `make clean` | Remove build artefacts (`cargo clean`)                  |
| `make flash` | Build and flash the board, then reset it                |
| `make debug` | Clean → build → flash → open GDB with breakpoint at `main` |

---

## Firmware overview

`src/main.rs` implements a 1 Hz LED blinker using a TIM2 interrupt:

1. Configures the RCC to run the system at 180 MHz from the 8 MHz HSE crystal.
2. Configures PG13 and PG14 as push-pull outputs.
3. Sets up TIM2 to fire an interrupt every second.
4. In the `TIM2` interrupt handler, toggles the two LEDs by switching between two states.
5. The main loop calls `wfi` (Wait For Interrupt) — the CPU sleeps between interrupts.

---

## VS Code

See [.vscode/README.md](./.vscode/README.md) for IDE-based debugging with the
Cortex-Debug extension.

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
