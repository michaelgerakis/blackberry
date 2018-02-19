extern crate serial;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate xmodem;

use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use serial::core::{BaudRate, CharSize, FlowControl, SerialDevice,
                   SerialPortSettings, StopBits};
use structopt::StructOpt;
use xmodem::Xmodem;

mod parsers;

use parsers::{parse_baud_rate, parse_flow_control, parse_stop_bits,
              parse_width};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i",
                help = "Input file (defaults to stdin if not set)",
                parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud",
                parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout",
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width",
                parse(try_from_str = "parse_width"),
                help = "Set data character width in bits",
                default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control",
                parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')",
                default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits",
                parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")] raw: bool,
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    let opt = Opt::from_args();
    let mut serial =
        serial::open(&opt.tty_path).expect("path points to invalid TTY");

    let mut buf = String::new();
    let stdin = io::stdin()
        .read_line(&mut buf)
        .expect("Please input a value.");

    println!("{}", stdin);
    println!("{}", buf);

    serial.write(&buf.into_bytes());
    serial.flush();
}
