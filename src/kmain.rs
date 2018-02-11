#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods)]
#![no_builtins]
#![no_std]

extern crate compiler_builtins;
extern crate volatile;

pub mod lang_items;
pub mod common;
pub mod timer;
pub mod gpio;
pub mod uart;

use gpio::GPIO;
use uart::UART;

#[no_mangle]
pub extern "C" fn kmain() {
    GPIO::cleanup();

    let mut mu = UART::new(270);

    loop {
        mu.write_byte(b'>');
        mu.write_byte(b' ');

        let byte = mu.read_byte();

        mu.write_byte(byte);

        mu.write_byte(0x0d);
        mu.write_byte(0x0a);
    }
}
