#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods)]
#![no_builtins]
#![no_std]

extern crate compiler_builtins;
extern crate volatile;

pub mod lang_items;
pub mod timer;
pub mod gpio;
mod common;

use gpio::GPIO;
use timer::spin_sleep_ms;

#[no_mangle]
pub extern "C" fn kmain() {
    GPIO::cleanup();

    let mut pins = [GPIO::new(5).into_output(), GPIO::new(19).into_output()];

    spin_sleep_ms(3000);

    loop {
        pins[0].set();
        pins[1].clear();
        spin_sleep_ms(300);
        pins[0].clear();
        pins[1].set();
        spin_sleep_ms(300);
    }
}
