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

use console::kprintln;
use pi::atags::Atags;
use pi::timer::spin_sleep_ms;

#[no_mangle]
pub extern "C" fn kmain() {
    spin_sleep_ms(2000);
    for val in Atags::get() {
        kprintln!("{:?}", val);
    }
}
