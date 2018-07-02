extern crate relay8x_lib;
extern crate docopt;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::{thread::sleep, time::Duration};

use relay8x_lib::{CardIndex, Relay8x, RelayIndex};

const USAGE: &'static str = "
relay8x

Usage:
  relay8x set --dev=<dev> [--card=<card> ...] [--relay=<relay> ...] <state>
  relay8x toggle --dev=<dev> [--card=<card> ...] [--relay=<relay> ...]
  relay8x reset --dev=<dev> [--card=<card> ...] [--relay=<relay> ...]
  relay8x (-h | --help)
  relay8x (-v | --version)
  
Commands:
  set   set specified relay 'on' or 'off', if no relay number is given all relays are set
  toggle    toggle specified relay,  if no relay number is given all relays are toggeled
  reset switch all or just one relay off to reach defined state again

Options:
  -h --help         Show this screen.
  -v --version      Show version.
  --dev=<dev>       path to serial device, e.g. /dev/ttyUSB0
  --relay=<relay>   number of relays (1..8), one flag per relay [default: 1 2 3 4 5 6 7 8]
  --card=<card>     number of card (1..x), one flag per card [default: 0]
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_set: bool,
    cmd_toggle: bool,
    cmd_reset: bool,
    flag_dev: String,
    flag_version: bool,
    flag_help: bool,
    flag_relay: Option<RelayIndex>,
    flag_card: Option<CardIndex>,
    arg_state: String,
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
        let mut relay = Relay8x::new(args.flag_dev, 1).expect("Failed to create new device");
        relay.init_device().expect("Failed to init device");
        // wait to give cards time to answer
        sleep(Duration::from_millis(10));
        // if flag_relay is none, all relays should be set
        let relay_numbers = args.flag_relay.unwrap_or_default();
        // if flag_card is none, all cards should be set
        let card_numbers = args.flag_card.unwrap_or_default();
        // map state argument to set or reset
        match args.arg_state.as_ref() {
            "on" => relay.set_relays(card_numbers, relay_numbers).expect("Failed to set relay"),
            "off" => relay.reset_relays(card_numbers, relay_numbers).expect("Failed to set relay"),
            _ => relay.reset_relays(card_numbers, relay_numbers).expect("Failed to set relay"),
        };

    } else if args.cmd_toggle {
        // open device
        let mut relay = Relay8x::new(args.flag_dev, 1).expect("Failed to create device");
        relay.init_device().expect("Failed to init device");
        // wait to give cards time to answer
        sleep(Duration::from_millis(10));
        // if flag is none, all relays should be toggeled
        let relay_numbers = args.flag_relay.unwrap_or_default();
        // if flag_card is none, all cards should be set
        let card_numbers = args.flag_card.unwrap_or_default();
        // do the toggle
        relay.toggle_relays(card_numbers, relay_numbers).expect("Failed to toggle relay");
    } else if args.cmd_reset {
        // open device
        let mut relay = Relay8x::new(args.flag_dev, 1).expect("Failed to create device");
        relay.init_device().expect("Failed to init device");
        // wait to give cards time to answer
        sleep(Duration::from_millis(10));
        // if flag is none, all relays should be reset
        let relay_numbers = args.flag_relay.unwrap_or_default();
        // if flag_card is none, all cards should be set
        let card_numbers = args.flag_card.unwrap_or_default();
        // do the switching, false = off
        relay.reset_relays(card_numbers, relay_numbers).expect("Failed to reset relay");
    } else {
        println!("I don't know what you want to do..");
    }
}

