#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods)]
#![no_builtins]
#![no_std]

extern crate compiler_builtins;

pub mod lang_items;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 600) {
        unsafe {
            asm!("nop" :::: "volatile");
        }
    }
}

unsafe fn set_addr_to_bit(addr: *mut u32, bit: u32, offset: u32) {
    addr.write_volatile(bit << offset);
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    set_addr_to_bit(GPIO_FSEL1, 0b1, 18);

    loop {
        set_addr_to_bit(GPIO_SET0, 0b1, 16);
        spin_sleep_ms(1000);
        set_addr_to_bit(GPIO_CLR0, 0b1, 16);
        spin_sleep_ms(1000);
    }
}
