#![feature(compiler_builtins_lib)]
#![feature(lang_items)]
#![feature(pointer_methods)]
#![feature(asm)]
#![feature(repr_align)]
#![feature(const_fn)]
#![feature(attr_literals)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]

extern crate core;
extern crate volatile;

pub mod lang_items;
pub mod mutex;
pub mod console;
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
