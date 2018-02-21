// These functions below provide definitions for symbols libcore
// expects which are not present on our bare metal target. You
// will not encounter linker errors until you use a part of
// libcore that references them, such as iterators in this program.
// In the future you may need to provide real implementations for
// these functions.

use console::kprintln;
use std;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(
    fmt: std::fmt::Arguments,
    file: &str,
    line: u32,
    col: u32,
) -> ! {
    kprintln!("FILE: {}", file);
    kprintln!("LINE: {}", line);
    kprintln!("COL: {}", col);

    kprintln!("{}", fmt);
    loop {}
}

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}
