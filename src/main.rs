extern crate serial;
extern crate docopt;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bytes;

use serial::prelude::*;
use docopt::Docopt;
use std::io;
use std::time::Duration;
use bytes::{BytesMut, BufMut};

const USAGE: &'static str = "
relais8x

Usage:
  relais8x --dev=<dev> [--relais=<relais>]
  relais8x (-h | --help)
  relais8x (-v | --version)
  
Commands:
  connect   connect to device and toggle relaise

Options:
  -h --help     Show this screen.
  -v --version     Show version.
  --dev=<dev>   name of serial device (TTYxx)
  --relaise=<relaise>   address of relaise (1..8)
";

#[derive(Debug, Deserialize)]
struct Args {
    //cmd_connect: bool,
    flag_dev: String,
    flag_relaise: String,
    flag_version: bool,
    flag_help: bool,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

fn main() {

    env_logger::init();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());  

    // check arguments
    if args.flag_version {
        println!("{}: {}", NAME, VERSION);
    } else if args.flag_help {
        println!("{}", USAGE);
    }

    let device_name = format!("/dev/{}", args.flag_dev);
    let mut port = serial::open(&device_name).unwrap();
    interact(&mut port).unwrap();
    

}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    try!(port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud19200));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));

    try!(port.set_timeout(Duration::from_millis(1000)));

    let mut buf =  BytesMut::with_capacity(4);
    buf.put_u8(8);
    buf.put_u8(1);
    buf.put_u8(1);
    buf.put_u8(8 ^ 1 ^ 1);

    println!("command: {:?}", buf);

    try!(port.write(&buf[..]));
    try!(port.read(&mut buf[..]));
    println!("response: {:?}", buf);

    Ok(())
}
