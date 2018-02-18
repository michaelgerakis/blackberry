use std::fmt;
use std::io;

use common::IO_BASE;
use gpio::{Function, GPIO};
use timer::current_time;
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
    timeout:   Option<u64>,
}

#[repr(u8)]
enum LsrStatus {
    DataReady = 1,
    TxAvailable = 1 << 5,
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

        UART {
            registers,
            timeout: None,
        }
    }

    pub fn set_read_timeout(&mut self, milliseconds: u32) {
        self.timeout = Some(milliseconds as u64);
    }

    pub fn write_byte(&mut self, byte: u8) {
        while !self.registers.lsr.has_mask(LsrStatus::TxAvailable as u32) {}

        self.registers.io.write(byte as u32);
    }

    pub fn has_byte(&self) -> bool {
        self.registers.lsr.has_mask(LsrStatus::DataReady as u32)
    }

    pub fn wait_for_byte(&self) -> Result<(), ()> {
        match self.timeout {
            Some(ms) => {
                let start_time = current_time();

                while current_time() > start_time + ms {
                    if self.has_byte() {
                        return Ok(());
                    }
                }

                return Err(());
            }
            None => {
                while !self.has_byte() {}
                return Ok(());
            }
        }
    }

    pub fn read_byte(&self) -> u8 {
        while !self.has_byte() {}

        self.registers.io.read() as u8
    }
}

impl fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            if *c == b'\n' {
                self.write_byte(b'\r');
            }

            self.write_byte(*c);
        }

        Ok(())
    }
}

impl io::Write for UART {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for b in buf {
            self.write_byte(*b);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

impl io::Read for UART {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read = 0;

        if self.wait_for_byte() == Ok(()) {
            while self.has_byte() && read < buf.len() {
                buf[read] = self.read_byte();
                read += 1;
            }
        }

        Ok(read)
    }
}
