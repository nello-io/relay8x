extern crate serial;
extern crate docopt;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use serial::prelude::*;
use docopt::Docopt;

const USAGE: &'static str = "
relaise8x

Usage:
  relaise8x connect --dev=<dev> --relaise=<relaise>
  relaise8x (-h | --help)
  relaise8x (-v | --version)
  
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
    cmd_connect: bool,
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

}
