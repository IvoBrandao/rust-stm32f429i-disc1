#![no_std]
#![no_main]

// startup crate provides the Reset handler, vector table, and panic handler.
use startup as _;

// -- Register addresses (STM32F429 Reference Manual) --------------------------

const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32; // AHB1 peripheral clock enable
const GPIOG_MODER: *mut u32 = 0x4002_1800 as *mut u32; // port mode register
const GPIOG_BSRR:  *mut u32 = 0x4002_1818 as *mut u32; // bit set/reset register

// -----------------------------------------------------------------------------

/// Application entry point called by the startup Reset handler.
#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // 1. Enable GPIOG peripheral clock (AHB1ENR bit 6)
        RCC_AHB1ENR.write_volatile(RCC_AHB1ENR.read_volatile() | (1 << 6));

        // 2. Set PG13 and PG14 to general-purpose output mode (MODER = 0b01)
        //    PG13 → MODER[27:26], PG14 → MODER[29:28]
        let moder = GPIOG_MODER.read_volatile() & !(0xF << 26);
        GPIOG_MODER.write_volatile(moder | (0b01 << 26) | (0b01 << 28));

        // 3. Alternate green (PG13) and red (PG14) at ~1 Hz
        //    The chip runs at 16 MHz HSI after reset; 4_000_000 NOPs ≈ 1 s.
        loop {
            // PG13 on, PG14 off  (BSRR upper 16 bits reset, lower 16 bits set)
            GPIOG_BSRR.write_volatile((1 << 13) | (1 << (14 + 16)));
            delay(4_000_000);

            // PG14 on, PG13 off
            GPIOG_BSRR.write_volatile((1 << 14) | (1 << (13 + 16)));
            delay(4_000_000);
        }
    }
}

#[inline(never)]
fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe { core::arch::asm!("nop") };
    }
}
