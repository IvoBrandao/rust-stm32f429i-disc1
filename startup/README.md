# startup

A bare-metal startup crate for the STM32F429ZI (ARM Cortex-M4F).

This crate replaces `cortex-m-rt`. It owns everything that must happen before `main` function is allowed to run: the vector table, the reset handler, and the linker script that glues it all together.

---

## Why does this crate exist?

When a Cortex-M microcontroller powers on, the CPU does **not** jump to `main`.
It does two things first:

1. Reads the **initial stack pointer** from word 0 of Flash (`0x0800_0000`).
2. Reads the **Reset handler address** from word 1 of Flash and jumps to it.

Everything from word 0 of Flash to the start of your code is called the
**vector table**. This crate builds that table, implements the Reset handler,
and hands control to your `main`.

---

## What does `core` provide?

Even though `Cargo.toml` has **zero external dependencies**, Rust always links
one built-in crate: **`core`**.

`core` is the dependency-free subset of the Rust standard library. It is
compiled for every target, including bare-metal embedded, and it provides:

| What you use                     | Where it lives           |
|------------------------          |--------------------------|
| `core::ptr::addr_of!`            | raw pointer from a static |
| `core::ptr::addr_of_mut!`        | mutable raw pointer   |
| `core::ptr::copy_nonoverlapping` | memcpy equivalent |
| `core::ptr::write_bytes`         | memset equivalent      |
| `core::panic::PanicInfo`         | panic handler type     |
| `core::arch::asm!`               | inline assembly          |

`write_volatile` in `src/main.rs` is a **method on a raw pointer** (`*mut u32`).
Raw pointer methods are also defined in `core::ptr`. When you write:

```rust
const GPIOG_MODER: *mut u32 = 0x4002_1800 as *mut u32;
GPIOG_MODER.write_volatile(value);
```

that is exactly the same as:

```rust
core::ptr::write_volatile(GPIOG_MODER, value);
```

`write_volatile` prevents the compiler from reordering or eliminating the
memory access. This is essential for hardware registers: without it, the
compiler might decide the write is "unnecessary" (since nothing in Rust ever
reads that address) and remove it entirely.

---

## File structure

```
startup/
├── Cargo.toml    — package manifest (no [dependencies])
├── build.rs      — copies link.x to OUT_DIR, emits linker flags
├── link.x        — linker script: section layout + symbol exports
└── src/
    └── lib.rs    — Reset handler + Cortex-M4 vector table + DefaultHandler
```

---

## Linker script (`link.x`)

The linker script controls how the compiled code is arranged in Flash and RAM.

### Memory regions

`link.x` does not define memory sizes itself. It starts with:

```ld
INCLUDE memory.x
```

`memory.x` lives in the application crate and describes the specific chip:

```ld
MEMORY
{
  FLASH  (rx)  : ORIGIN = 0x08000000, LENGTH = 2048K
  RAM    (xrw) : ORIGIN = 0x20000000, LENGTH = 192K
  CCMRAM (rw)  : ORIGIN = 0x10000000, LENGTH = 64K
}
```

### Sections

| Section        | Region | Purpose                                            |
|----------------|--------|----------------------------------------------------|
| `.vector_table`| FLASH  | Stack pointer init + all exception vectors         |
| `.text`        | FLASH  | Compiled machine code                              |
| `.rodata`      | FLASH  | String literals, `const` values                   |
| `.data`        | RAM    | Initialised globals (copied from Flash by Reset)   |
| `.bss`         | RAM    | Zero-initialised globals (zeroed by Reset)         |

### Symbols exported to Rust

The linker script marks the boundaries of `.data` and `.bss` so the Reset
handler knows what to copy and zero:

| Symbol      | Meaning                                         |
|-------------|-------------------------------------------------|
| `__sdata`   | Start address of `.data` in RAM                 |
| `__edata`   | End address of `.data` in RAM                   |
| `__sidata`  | Load address of `.data` in Flash (copy source)  |
| `__sbss`    | Start address of `.bss` in RAM                  |
| `__ebss`    | End address of `.bss` in RAM                    |

### Vector table layout

The vector table must start at `ORIGIN(FLASH)` (`0x0800_0000`):

```
Offset  Content
------  -------
0x00    Initial stack pointer  = top of RAM  (a raw LONG value, not code)
0x04    Reset handler address               (.vector_table.reset_vector)
0x08    NMI handler                         (.vector_table.exceptions[0])
0x0C    HardFault handler                   (.vector_table.exceptions[1])
  ...
0x3C    SysTick handler                     (.vector_table.exceptions[13])
```

After the 16 Cortex-M system vectors, the table continues with
device-specific IRQ vectors (timers, UARTs, etc.) — not yet implemented here.

---

## Reset handler (`src/lib.rs`)

```
power-on / reset
      │
      ▼
CPU reads word 0 of Flash → loads into stack pointer register (SP)
CPU reads word 1 of Flash → jumps to Reset()
      │
      ▼
Reset() copies .data from Flash to RAM
Reset() zeroes .bss in RAM
      │
      ▼
main() — your code runs
```

### Why copy `.data`?

Flash is read-only at runtime. Initialised globals live in Flash at build
time (their initial values are baked into the binary). Before `main` runs,
the Reset handler copies them to the RAM addresses where your code will
actually read and write them.

### Why zero `.bss`?

The C and Rust standards guarantee that global variables without an explicit
initialiser are zero. RAM has no such guarantee after power-on, so the Reset
handler explicitly sets the whole `.bss` region to `0x00`.

---

## DefaultHandler

Every unused exception vector points to `DefaultHandler`, which spins in an
infinite loop. In a real project you would replace specific entries with real
handlers (e.g. `HardFault`, `SysTick`, peripheral IRQs).

---

## How the application uses this crate

`src/main.rs` imports the startup crate so the linker includes it:

```rust
use startup as _;   // pull in Reset, vector table, DefaultHandler
```

The `main` function must be `extern "C"` and `#[no_mangle]` so the Reset
handler can find it by name at link time:

```rust
#[no_mangle]
pub extern "C" fn main() -> ! {
    // your code here
}
```

The application crate never calls `Reset` directly — the hardware does.
