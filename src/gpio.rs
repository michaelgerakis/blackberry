use common::IO_BASE;
use core::marker::PhantomData;
use volatile::{ReadVolatile, Reserved, Volatile, WriteVolatile};
use volatile::prelude::*;

const GPIO_BASE: usize = IO_BASE + 0x200000;

#[repr(u8)]
pub enum Function {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

#[repr(C)]
struct GPIORegisters {
    fsel:  [Volatile<u32>; 6],
    _res0: Reserved<u32>,
    set:   [WriteVolatile<u32>; 2],
    _res1: Reserved<u32>,
    clr:   [WriteVolatile<u32>; 2],
    _res2: Reserved<u32>,
    lvl:   [ReadVolatile<u32>; 2],
}

pub enum Uninitialized {}
pub enum Output {}
pub enum Input {}
pub enum Alternate {}

pub struct GPIO<State> {
    state:     PhantomData<State>,
    pin:       u8,
    registers: &'static mut GPIORegisters,
}

impl GPIO<Uninitialized> {
    pub fn cleanup() {
        for i in 0..54 {
            GPIO::new(i).into_output().clear();
        }
    }

    pub fn new(pin: u8) -> GPIO<Uninitialized> {
        if pin > 53 {
            panic!("Gpio::new(): pin {} exceeds maximum of 53", pin);
        }

        GPIO {
            pin,
            state: PhantomData,
            registers: unsafe { &mut *(GPIO_BASE as *mut GPIORegisters) },
        }
    }

    #[inline(always)]
    pub fn into_output(self) -> GPIO<Output> {
        return self.transition(Function::Output);
    }

    #[inline(always)]
    pub fn into_input(self) -> GPIO<Input> {
        return self.transition(Function::Input);
    }

    #[inline(always)]
    pub fn into_alt(self, alt: Function) -> GPIO<Alternate> {
        return self.transition(alt);
    }
}

impl<T> GPIO<T> {
    fn transition<R>(self, function: Function) -> GPIO<R> {
        let offset = (self.pin % 10) * 3;
        let reg = self.pin / 10;

        self.registers.fsel[reg as usize].and_mask(!((0b111 as u32) << offset));
        self.registers.fsel[reg as usize].or_mask((function as u32) << offset);

        GPIO {
            pin:       self.pin,
            state:     PhantomData,
            registers: self.registers,
        }
    }
}

impl<Output> GPIO<Output> {
    pub fn set(&mut self) {
        let reg = self.pin / 32;
        let offset = self.pin % 32;

        self.registers.set[reg as usize].write(0b1 << offset);
    }

    pub fn clear(&mut self) {
        let reg = self.pin / 32;
        let offset = self.pin % 32;

        self.registers.clr[reg as usize].write(0b1 << offset);
    }
}
