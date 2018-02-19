#![feature(pointer_methods)]

extern crate serial;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate xmodem;

use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;
use std::time::Duration;

use serial::core::{BaudRate, CharSize, FlowControl, SerialDevice,
                   SerialPortSettings, StopBits};
use structopt::StructOpt;
use xmodem::{Progress, Xmodem};

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

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
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

    let opt = Opt::from_args();
    let mut serial =
        serial::open(&opt.tty_path).expect("path points to invalid TTY");

    let mut settings =
        serial.read_settings().expect("Failed to load settings.");
    settings
        .set_baud_rate(opt.baud_rate)
        .expect("Invalid baud rate.");
    settings.set_char_size(opt.char_width);
    settings.set_flow_control(opt.flow_control);
    settings.set_stop_bits(opt.stop_bits);
    serial
        .write_settings(&settings)
        .expect("Failed to apply serial settings");

    serial
        .set_timeout(Duration::from_secs(opt.timeout))
        .expect("Invalid timeout");

    let sent_raw = match opt.input {
        Some(file) => send_to_serial(
            BufReader::new(File::open(file).expect("File open failed")),
            &mut serial,
            opt.raw,
        ),
        None => send_to_serial(BufReader::new(stdin()), &mut serial, opt.raw),
    };

    match sent_raw {
        Ok(n) => println!("wrote {} bytes to input", n),
        Err(e) => println!("error: {:?}", e),
    }
}

fn send_to_serial<I: BufRead>(
    mut input: I,
    serial: &mut serial::SerialPort,
    raw: bool,
) -> Result<usize, std::io::Error> {
    match raw {
        true => serial.write(input.fill_buf()?),
        false => Xmodem::transmit_with_progress(input, serial, progress_fn),
    }
}

fn progress_fn(progress: Progress) {
    use std::io::Write;
    static mut LAST_PKT: u8 = 0;

    match progress {
        Progress::Started => {
            println!("Transmission started:");
        }
        Progress::Waiting => println!("Waiting for initial NAK"),
        Progress::Packet(p) => {
            if unsafe { p == LAST_PKT } {
                print!("@");
            } else {
                unsafe { LAST_PKT = p };
                print!("#")
            }
            std::io::stdout().flush().unwrap();
        }
        Progress::Terminated(p) => {
            println!("");
            println!("| wrote {} packets", p);
        }
    }
}
