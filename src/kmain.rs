#![feature(compiler_builtins_lib)]
#![feature(exclusive_range_pattern)]
#![feature(lang_items)]
#![feature(pointer_methods)]
#![feature(asm)]
#![feature(repr_align)]
#![feature(const_fn)]
#![feature(attr_literals)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]

extern crate core;
extern crate pi;
extern crate stack_vec;
extern crate volatile;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

use pi::gpio::GPIO;
use pi::uart::UART;
use shell::shell;

#[no_mangle]
pub extern "C" fn kmain() {
    GPIO::cleanup();

    let mut uart = UART::new(270);

    while !uart.has_byte() {
        uart.write_byte(b'>');
        uart.write_byte(b' ');
        uart.write_byte(b'\r');
    }

    shell(">");
}
