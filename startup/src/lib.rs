//! Bare-metal startup for Cortex-M4 (STM32F429ZI).
//!
//! Responsibilities:
//!   1. Place the vector table at the very start of Flash.
//!   2. Implement the Reset handler that initialises RAM and calls `main`.
//!   3. Provide a DefaultHandler for all unhandled exceptions.
//!
//! The linker script (link.x) defines the symbols used here:
//! __sbss, __ebss, __sdata, __edata, __sidata.

#![no_std]

use core::panic::PanicInfo;

// Symbols injected by link.x that mark section boundaries.
extern "C" {
    static mut __sbss: u8; // start of .bss in RAM
    static mut __ebss: u8; // end   of .bss in RAM
    static mut __sdata: u8; // start of .data in RAM
    static mut __edata: u8; // end   of .data in RAM
    static __sidata: u8; // load address of .data in Flash (copy source)
}

// User application entry point defined in the application crate.
extern "C" {
    fn main() -> !;
}

/// Reset handler — the first code that runs after power-on or reset.
///
/// 1. Copies initialised globals (.data) from Flash to RAM.
/// 2. Zero-fills uninitialised globals (.bss) in RAM.
/// 3. Calls the user `main`.
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    use core::ptr::{addr_of, addr_of_mut};

    // Copy .data from Flash (LMA) to RAM (VMA)
    let data_len = addr_of!(__edata) as usize - addr_of!(__sdata) as usize;
    core::ptr::copy_nonoverlapping(addr_of!(__sidata), addr_of_mut!(__sdata), data_len);

    // Zero-fill .bss
    let bss_len = addr_of!(__ebss) as usize - addr_of!(__sbss) as usize;
    core::ptr::write_bytes(addr_of_mut!(__sbss), 0, bss_len);

    main()
}

/// Catches any exception that has no dedicated handler.
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler() {
    loop {}
}

// -- Vector table -------------------------------------------------------------
//
// Cortex-M4 layout (ARM DDI 0403):
//   Word  0   : initial stack pointer  (LONG in link.x)
//   Word  1   : Reset                  (.vector_table.reset_vector)
//   Words 2-15: exception handlers     (.vector_table.exceptions)

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

type Handler = unsafe extern "C" fn();

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Option<Handler>; 14] = [
    Some(DefaultHandler), //  2 — NMI
    Some(DefaultHandler), //  3 — HardFault
    Some(DefaultHandler), //  4 — MemManage
    Some(DefaultHandler), //  5 — BusFault
    Some(DefaultHandler), //  6 — UsageFault
    None,                 //  7 — Reserved
    None,                 //  8 — Reserved
    None,                 //  9 — Reserved
    None,                 // 10 — Reserved
    Some(DefaultHandler), // 11 — SVCall
    Some(DefaultHandler), // 12 — DebugMon
    None,                 // 13 — Reserved
    Some(DefaultHandler), // 14 — PendSV
    Some(DefaultHandler), // 15 — SysTick
];

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
