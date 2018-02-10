pub trait Readable<T> {
    #[inline(always)]
    fn inner(&self) -> *const T;

    #[inline(always)]
    fn read(&self) -> T { unsafe { ::core::ptr::read_volatile(self.inner()) } }

    #[inline(always)]
    fn has_mask(&self, mask: T) -> bool
    where
        T: ::core::ops::BitAnd<Output = T>,
        T: PartialEq + Copy,
    {
        (self.read() & mask) == mask
    }
}

pub trait Writeable<T> {
    #[inline(always)]
    fn inner(&mut self) -> *mut T;

    #[inline(always)]
    fn write(&mut self, val: T) {
        unsafe {
            ::core::ptr::write_volatile(self.inner(), val);
        }
    }
}

pub trait ReadableWriteable<T>: Readable<T> + Writeable<T>
where
    T: ::core::ops::BitAnd<Output = T>,
    T: ::core::ops::BitOr<Output = T>,
{
    fn and_mask(&mut self, mask: T) {
        let init_val = self.read();
        self.write(init_val & mask);
    }

    fn or_mask(&mut self, mask: T) {
        let init_val = self.read();
        self.write(init_val | mask);
    }
}
