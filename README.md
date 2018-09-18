# relay8x

Library and utility command line tool to communicate with [this relaise cards](https://www.conrad.de/de/relaiskarte-baustein-conrad-components-197720-12-vdc-24-vdc-197720.html#downloadcenter) over serial and/or USB interface

Install the binary use `cargo install relay8x` and read `relay8x --help` for details on useage.

## Features

- [x] set specific or all relays on or off
- [x] toggle specific or all relays on or off
- [x] reset (=turn off) specific or all relays
- [x] multiple cards on one serial device
- [ ] run custom command

## Documentation

details about communication protocol are in this [pdf](DOC_8FACH_RELAISKARTE_24V_7A_de_en_fr_nl.pdf)

## Wiring / Setup

### One card

- Jumper JP1 in position `1-2`
- connect to USB

### Cascading

- Jumper JP1 in position `2-3` for all cards but the last
- Jumper JP1 at last card in position `1-2`
- connect `GND` terminals with following card
- connect `Txb` of preceeding card with `Rxa` of following card
- connect `Rxb` of preceeding card with `Txa` of following card

### Windows

Driver works fine with Linux, during testing intermitted issues with stacked relaise cards appeared only on Windows.