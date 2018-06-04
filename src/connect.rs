use serial::prelude::*;
use serial;
//use serial_unix::*;
use std::io;
use std::time::Duration;
use bytes::{BytesMut, BufMut};

pub struct Relay8x {
    device_name: String,
    address: u8,
    port: SerialPort,
}

impl Relay8x {

    pub fn new(device_name: String, address: u8) -> Self {
        let device_name = format!("/dev/{}", device_name);
        Self {
            port: serial::open(&device_name).unwrap(),
            device_name: device_name,
            address: address,
        }
    }

    pub fn init_device(&self) -> io::Result<()> {
        self.configure_port()?;
        // init relaycard
        let mut cmd = BytesMut::with_capacity(4);
        let cmd_no = 1; // first byte: command init device
        cmd.put_u8(cmd_no);
        cmd.put_u8(self.address); // second byte: address of card
        cmd.put_u8(0);  // third: dont care
        cmd.put_u8(cmd_no ^ self.address ^ 0); // fourth: XOR

        self.port.write(&cmd[..])?;

        self.port.read(&mut cmd[..])?;
        debug!("Response: {:?}", cmd);
        // TODO return response and check if ok

        Ok(())
    }

    fn configure_port(&self) -> io::Result<()> {
        
        self.port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud19200)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        self.port.set_timeout(Duration::from_millis(1000))?;

        Ok(())
    }

    pub fn set_relay(&self, number: u8, state: bool) -> io::Result<()> {
        let mut cmd = BytesMut::with_capacity(4);

        let on_off = if state { // on
            6
        } else { // off
            7
        };
        cmd.put_u8(on_off);
        cmd.put_u8(self.address);
        cmd.put_u8(number);
        cmd.put_u8(on_off ^ self.address ^ number);

        self.port.write(&cmd[..])?;

        Ok(())
    }
}