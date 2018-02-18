use console::{kprint, kprintln, CONSOLE};
use stack_vec::StackVec;
use std::str::from_utf8;

const BS: u8 = 0x08;
const DEL: u8 = 0x7F;
const BEL: u8 = 0x07;
const LF: u8 = 0x0A;
const CR: u8 = 0x0D;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first
    /// argument.
    fn path(&self) -> &str { self.args.as_slice()[0] }

    fn exec(&self) {
        match self.path() {
            path if path.as_bytes() == b"echo" => {
                for arg in &self.args.as_slice()[1..] {
                    kprint!("{} ", arg);
                }

                kprintln!("");
            }
            path => {
                kprintln!("unknown command: {}", path);
            }
        }
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    loop {
        kprint!("{} ", prefix);

        match Command::parse(
            read_line(StackVec::new(&mut [0x8; 512])),
            &mut [""; 64],
        ) {
            Err(Error::Empty) => continue,
            Err(Error::TooManyArgs) => kprintln!("error: too many arguments"),
            Ok(cmd) => cmd.exec(),
        }
    }
}

fn read_line(mut buf: StackVec<u8>) -> &str {
    loop {
        let byte = CONSOLE.lock().read_byte();

        match byte {
            // Break from command capture with new lines
            CR | LF => {
                kprintln!("");
                break;
            }
            // Backspace deletes most recent character if we are not empty
            BS | DEL => if !buf.is_empty() {
                kprint!("{} {}", BS as char, BS as char);
                buf.pop();
            },
            // Ring the bell on non printable characters
            0...31 => {
                kprint!("{}", BEL as char);
            }
            // Else write byte and push on stack
            _ => match buf.push(byte) {
                // Unless we have 512 characters already..
                Err(_) => {
                    kprint!("{}", BEL as char);
                }
                Ok(_) => {
                    kprint!("{}", byte as char);
                }
            },
        }
    }

    from_utf8(buf.into_slice()).unwrap()
}
