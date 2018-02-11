use common::IO_BASE;
use gpio::{Function, GPIO};
use volatile::{ReadVolatile, Volatile};
use volatile::prelude::*;

/// The `AUXENB` register from page 9 of the BCM2837 documentation.
const AUX_ENABLES: *mut Volatile<u8> =
    (IO_BASE + 0x215004) as *mut Volatile<u8>;

const MU_BASE: usize = IO_BASE + 0x00215040;

#[repr(C)]
struct Registers {
    io:      Volatile<u32>, // write/read to an from UART FIFOs
    ier:     Volatile<u32>, // interrupt enable
    iir:     Volatile<u32>, // interrupt statu
    lcr:     Volatile<u32>, // line data control
    mcr:     Volatile<u32>, // modem control
    lsr:     ReadVolatile<u32>, // line data status
    msr:     ReadVolatile<u32>, // modem status
    scratch: Volatile<u32>,
    cntl:    Volatile<u32>,     // various extra features
    stat:    ReadVolatile<u32>, // extra internal status information
    baud:    Volatile<u32>,     // direct access to the 16-bit baudrate counter
}

pub struct UART {
    registers: &'static mut Registers,
}

impl UART {
    pub fn new(baud_rate: u16) -> UART {
        let registers = unsafe {
            // enable mini uart
            (*AUX_ENABLES).or_mask(0b1);
            &mut *(MU_BASE as *mut Registers)
        };

        // set pins 14 and 15 to TXD0/RXD0
        GPIO::new(14).into_alt(Function::Alt5);
        GPIO::new(15).into_alt(Function::Alt5);

        // Set data size to 8 bits
        registers.lcr.write(0b11);
        // Set baud rate to the divisor given
        registers.baud.write(baud_rate as u32);
        // Enable tx and rx
        registers.cntl.or_mask(0b11);

        UART { registers }
    }

    pub fn write_byte(&mut self, byte: u8) {
        while !self.registers.lsr.has_mask(0b1 << 5) {}

        self.registers.io.write(byte as u32);
    }

    pub fn read_byte(&self) -> u8 {
        while !self.registers.lsr.has_mask(0b1) {}

        self.registers.io.read() as u8
    }
}
