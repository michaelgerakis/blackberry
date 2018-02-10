use common::IO_BASE;
use volatile::{ReadVolatile, Reserved};
use volatile::prelude::*;

const TIMER_BASE: usize = IO_BASE + 0x3000;

#[repr(C)]
struct TimerRegisters {
    _res0: Reserved<u32>,
    clo:   ReadVolatile<u32>,
    chi:   ReadVolatile<u32>,
}

fn current_time() -> u64 {
    let registers = unsafe { &*(TIMER_BASE as *const TimerRegisters) };
    (registers.chi.read() as u64) << 32 | registers.clo.read() as u64
}

pub fn spin_sleep_us(us: u64) {
    let start_time = current_time();

    while current_time() < start_time + us {}
}

pub fn spin_sleep_ms(ms: u64) { spin_sleep_us(ms * 1000); }
