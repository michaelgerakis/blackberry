#![no_std]

mod traits;
#[macro_use]
mod macros;

pub use traits::*;

pub mod prelude {
    pub use super::{Readable, ReadableWriteable, Writeable};
}

#[repr(C)]
pub struct Reserved<T>(T);

#[repr(C)]
pub struct ReadVolatile<T>(T);
readable!(ReadVolatile, |self| &self.0);

#[repr(C)]
pub struct WriteVolatile<T>(T);
writable!(WriteVolatile, |self| &mut self.0);

#[repr(C)]
pub struct Volatile<T>(T);
readable!(Volatile, |self| &self.0);
writable!(Volatile, |self| &mut self.0);
readable_writable!(Volatile);

#[cfg(test)]
mod tests {
    use super::{ReadVolatile, Reserved, Volatile};
    use traits::{Readable, Writeable};

    struct Register {
        _res:   [Reserved<u32>; 2],
        _read:  ReadVolatile<u32>,
        _write: Volatile<u32>,
    }

    #[test]
    fn read_from_ptr() {
        let val = [5, 15, 8, 6];
        let val_ptr: *const [u32] = &val;
        let reg: &mut Register = unsafe { &mut *(val_ptr as *mut Register) };
        assert_eq!(val[2], reg._read.read());
    }

    #[test]
    fn write_to_ptr() {
        let val = [5, 15, 8, 6];
        let val_ptr: *const [u32] = &val;
        let reg: &mut Register = unsafe { &mut *(val_ptr as *mut Register) };
        reg._write.write(8);

        assert_eq!(val[3], 8);
    }
}
