/// A raw `ATAG` as laid out in memory.
#[repr(C)]
pub struct Atag {
    pub size: u32,
    pub tag:  Values,
    pub kind: AtagKind,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Values {
    NONE = 0x0,
    CORE = 0x54410001,
    MEM = 0x54410002,
    CMD = 0x54410009,
}

impl Atag {
    pub fn next(&self) -> Option<&Atag> {
        match self.tag {
            Values::NONE => None,
            _ => {
                let ptr = self as *const Atag as *const u32;

                Some(unsafe {
                    &*(ptr.offset(self.size as isize) as *const Atag)
                })
            }
        }
    }
}

#[repr(C)]
pub union AtagKind {
    pub core: AtagCore,
    pub mem:  AtagMem,
    pub cmd:  AtagCmd,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtagCore {
    pub flags:     u32,
    pub page_size: u32,
    pub root_dev:  u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtagMem {
    pub size:  u32,
    pub start: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtagCmd {
    pub value: u8,
}
