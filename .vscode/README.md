# VS Code Debug Configuration — STM32F429ZI

This directory contains the VS Code debug configuration for the STM32F429I-DISC1 board.
Debugging goes through OpenOCD and the on-board ST-Link v2.1 probe.

---

## Required extensions

Install both extensions before attempting to debug:

```sh
code --install-extension rust-lang.rust-analyzer
code --install-extension marus25.cortex-debug
```

Or search for them in the Extensions panel:

- **rust-analyzer** (`rust-lang.rust-analyzer`) — Rust language support
- **Cortex-Debug** (`marus25.cortex-debug`) — ARM Cortex-M debug adapter

---

## Required tools

Cortex-Debug needs `gdb` and the ARM binutils (`arm-none-eabi-nm`, `arm-none-eabi-objdump`)
for full symbol classification. Install them with Homebrew:

```sh
# OpenOCD — on-chip debugger
brew install openocd

# Homebrew GDB (configured as gdbPath in launch.json)
brew install gdb

# ARM binutils for nm/objdump symbol classification
brew tap ArmMbed/homebrew-formulae
brew install arm-none-eabi-gcc
```

> **Why not `arm-none-eabi-gdb`?** It is not available as a standalone Homebrew package.
> `launch.json` sets `"gdbPath": "gdb"` to use the Homebrew `gdb` instead, which supports
> ARM targets on macOS.

---

## SVD file (peripheral register viewer)

The SVD file lets Cortex-Debug show live peripheral register values in the
**Cortex Peripherals** panel during a debug session.

1. Download the STM32F4 SVD pack from ST:
   `https://www.st.com/resource/en/svd/stm32f4_svd.zip`
2. Extract it and copy `STM32F429.svd` into `.vscode/`:

   ```text
   .vscode/STM32F429.svd
   ```

   `launch.json` already points to this path via `"svdFile"`.

The SVD file is optional — debugging works without it, you just won't see peripheral registers.

---

## How to start a debug session

1. Connect the board via the ST-Link USB port (the micro-USB closest to the reset button).
2. Open the **Run and Debug** panel (`Cmd+Shift+D`).
3. Select **Debug (OpenOCD)** from the dropdown.
4. Press **F5** or click the green play button.

VS Code will:

1. Run `cargo build` via the `Cargo Build (debug)` pre-launch task.
2. Start OpenOCD and connect to the board over ST-Link.
3. Flash the firmware.
4. Halt execution at `main`.

---

## Configuration notes

| Setting          | Value                   | Reason                                           |
|------------------|-------------------------|--------------------------------------------------|
| `gdbPath`        | `gdb`                   | Homebrew gdb; `arm-none-eabi-gdb` not on macOS  |
| `configFiles[0]` | `interface/stlink.cfg`  | Replaces deprecated `stlink-v2-1.cfg`           |
| `configFiles[1]` | `target/stm32f4x.cfg`  | STM32F4 family target                            |
| `cpuFrequency`   | `180000000`             | Sysclk = 180 MHz (from 8 MHz HSE)               |
| `swoFrequency`   | `2000000`               | SWO pin baud rate for ITM output                |

---

## ITM output

When the SWO pin is wired (it is on the STM32F429I-DISC1 via the ST-Link), ITM port 0
output appears in the VS Code **Output** panel under `SWO: ITM [port: 0, type: console]`.

---

## Committing these files

VS Code files are gitignored by default. To share this configuration:

```sh
git add -f .vscode/launch.json
git add -f .vscode/tasks.json
git add -f .vscode/extensions.json
```

Do **not** commit `.vscode/STM32F429.svd` — it is large and redistributable from ST directly.
