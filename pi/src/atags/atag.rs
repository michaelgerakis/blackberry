use atags::raw;
use std::{slice, str};

#[derive(Debug, Copy, Clone)]
pub enum Atag {
    Core(raw::AtagCore),
    Mem(raw::AtagMem),
    Cmd(&'static str),
    None,
}

impl<'a> From<&'a raw::Atag> for Atag {
    fn from(atag: &raw::Atag) -> Atag {
        unsafe {
            match (atag.tag, &atag.kind) {
                (raw::Values::CORE, &raw::AtagKind { core }) => {
                    Atag::from(core)
                }
                (raw::Values::MEM, &raw::AtagKind { mem }) => Atag::from(mem),
                (raw::Values::CMD, &raw::AtagKind { ref cmd }) => Atag::Cmd(
                    str::from_utf8(slice::from_raw_parts(
                        &cmd.value as *const u8,
                        atag.size as usize,
                    )).unwrap(),
                ),
                _ => Atag::None,
            }
        }
    }
}
impl From<raw::AtagCore> for Atag {
    fn from(core: raw::AtagCore) -> Atag { Atag::Core(core) }
}

impl From<raw::AtagMem> for Atag {
    fn from(mem: raw::AtagMem) -> Atag { Atag::Mem(mem) }
}
