extern crate serial;
extern crate serial_unix;
extern crate docopt;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bytes;
#[macro_use]
extern crate log;

use serial::prelude::*;
use docopt::Docopt;
use std::io;
use bytes::{BytesMut, BufMut};

mod connect;
use connect::*;

const USAGE: &'static str = "
relais8x

Usage:
  relais8x set --dev=<dev> [--relay=<relay>] on
  relais8x set --dev=<dev> [--relay=<relay>] off
  relais8x reset --dev=<dev>
  relais8x (-h | --help)
  relais8x (-v | --version)
  
Commands:
  set   set specified relay on or off, if no relay number is given all relays are set
  toggle    toggle specified relay,  if no relay number is given all relays are toggeled

Options:
  -h --help     Show this screen.
  -v --version     Show version.
  --dev=<dev>   name of serial device (TTYxxxx)
  --relay=<relay>   address of relay (1..8)
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_set: bool,
    cmd_toggle: bool,
    flag_dev: String,
    flag_relay: u8,
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
    } else if args.cmd_set {
        // open device, address of relay is always 1 as for now
        let relay = Relay8x::new(args.flag_dev, 1);
        relay.init_device();


    }
}

