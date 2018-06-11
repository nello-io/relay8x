use serial::prelude::*;
use std::io;
use std::io::{Error, ErrorKind};
use std::time::Duration;
use bytes::{BytesMut, BufMut};
use std::rc::Rc;

pub struct Relay8x {
    address: u8,
    port: Rc<SerialPort>,
}

impl Relay8x {

    /// constructor for a new Relay Card
    pub fn new(device_name: String, address: u8) -> io::Result<Self> {
        
        Ok(Self {
            port: Rc::new(::serial::open(&device_name)?),
            address: address,
        })
    }

    /// initialise device with correct params
    /// sets device address, function can be used to re-set it
    pub fn init_device(&mut self) -> io::Result<BytesMut> {

        let port = Rc::get_mut(&mut self.port).unwrap();
        Relay8x::configure_device(port)?;
        
        port.set_timeout(Duration::from_millis(1000))?;

        // init relaycard
        let mut cmd = BytesMut::with_capacity(4);
        let cmd_no = 1; // first byte: command init device
        cmd.put_u8(cmd_no);
        cmd.put_u8(self.address); // second byte: address of card
        cmd.put_u8(0);  // third: dont care
        cmd.put_u8(cmd_no ^ self.address ^ 0); // fourth: XOR

        debug!("Init address: {}", self.address);
        debug!("Init command: {:?}", &cmd);
        port.write(&cmd[..])?;
        port.read(&mut cmd[..])?;
        debug!("Response init: {:?}", &cmd);
        
        Ok(cmd)
    }

    /// private function for port settings
    fn configure_device(port: &mut SerialPort) -> io::Result<()> {
        
        port.reconfigure(&|settings| {
            settings.set_baud_rate(::serial::Baud19200)?;
            settings.set_char_size(::serial::Bits8);
            settings.set_parity(::serial::ParityNone);
            settings.set_stop_bits(::serial::Stop1);
            settings.set_flow_control(::serial::FlowNone);
            Ok(())
        })?;

        port.set_timeout(Duration::from_millis(1000))?;
        
        Ok(())
    }

    /// switch arbitrary relays on or off
    /// numbers: Vector containing all relay numbers (1..8)
    /// state: true for switching on, false for off
    pub fn set_relays(&mut self, numbers: Vec<u8>, state: bool) -> io::Result<BytesMut> {
        self.init_device()?;
        let port = Rc::get_mut(&mut self.port).unwrap();
        let mut cmd = BytesMut::with_capacity(4);
        let on_off = if state { // on
            6
        } else { // off
            7
        };
        cmd.put_u8(on_off);
        cmd.put_u8(self.address);
        let mut relay_bin = 0b00000000;
        numbers.iter().rev().for_each(|x| {
            relay_bin |= (1 << (x-1)) as u8;
        });
        cmd.put_u8(relay_bin);
        cmd.put_u8(on_off ^ self.address ^ relay_bin);

        debug!("{:?} => {:08b}", numbers, relay_bin);
        debug!("{:?}", cmd);

        port.write(&cmd[..])?;
        let sent_cmd = cmd.clone();
        port.read(&mut cmd[..])?;
        debug!("Set Relays response: {:?}", cmd);
        Relay8x::check_response(&cmd, &sent_cmd)?;
        Ok(cmd)
    }

    /// toggle aribtrary relays
    /// numbers: vector containing all relay numbers (1..8)
    pub fn toggle_relays(&mut self, numbers: Vec<u8>) -> io::Result<BytesMut> {

        self.init_device()?;
        let port = Rc::get_mut(&mut self.port).unwrap();

        let mut cmd = BytesMut::with_capacity(4);
        // toggle is command no 8
        cmd.put_u8(8); 
        cmd.put_u8(self.address);
        let mut relay_bin = 0;
        numbers.iter().rev().for_each(|x| {
            // has numbers to be sorted?
            relay_bin |= (1 << (x-1)) as u8;
        });
        cmd.put_u8(relay_bin);
        cmd.put_u8(8 ^ self.address ^ relay_bin);

        debug!("{:?} => {:08b}", numbers, relay_bin);
        debug!("command {:?}", cmd);

        port.write(&cmd[..])?;
        let sent_cmd = cmd.clone();
        port.read(&mut cmd[..])?;
        debug!("Set Relays response: {:?}", cmd);
        Relay8x::check_response(&cmd, &sent_cmd)?;
        
        Ok(cmd)
    }

    fn check_response(msg: & BytesMut, sent_msg: &BytesMut) -> io::Result<()> {
        
        // check first byte
        let checker_byte = sent_msg.get(0).unwrap_or(&1);
        let checked_bytes = msg.get(0).unwrap_or(&1);
        if *checked_bytes != !checker_byte  {
            return Err(Error::new(ErrorKind::Other, format!("Bad first byte: is {}, should be {}", checked_bytes, !checker_byte)))
        }
        // second byte: adress
        if msg.get(1).unwrap_or(&0) != sent_msg.get(1).unwrap_or(&1) {
            return Err(Error::new(ErrorKind::Other, format!("Wrong Adress: {}", msg.get(1).unwrap())))
        }
        // last byte: XOR
        if *msg.get(3).unwrap_or(&0) != (*msg.get(0).unwrap_or(&1) ^ *msg.get(1).unwrap_or(&0) ^ *msg.get(2).unwrap_or(&0)) {
            return Err(Error::new(ErrorKind::Other, "XOR in last byte is wrong"))
        }
        debug!("Check ok");
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn connect_to_card() {    
        let mut relay = Relay8x::new(String::from("/dev/ttyUSB0"), 1).expect("Failed to connect to device");
        let init_response = relay.init_device().expect("Failed to init device");
        let expected_res = BytesMut::from(vec![254, relay.address, 254, 254^relay.address^254]);
        assert_eq!(init_response, expected_res);
    }
}