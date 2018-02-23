mod raw;
mod atag;

/// The address at which the firmware loads the ATAGS
const ATAG_BASE: usize = 0x100;

pub struct Atags {
    pub ptr: &'static raw::Atag,
}

impl Atags {
    pub fn get() -> Atags {
        Atags {
            ptr: unsafe { &*(ATAG_BASE as *const raw::Atag) },
        }
    }
}

impl Iterator for Atags {
    type Item = atag::Atag;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ptr.tag {
            raw::Values::NONE => None,
            _ => self.ptr.next().map(|raw| {
                let res = atag::Atag::from(self.ptr);

                self.ptr = raw;

                res
            }),
        }
    }
}
