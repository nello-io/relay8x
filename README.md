# relay8x

small utility command line tool to communicate with [relaise cards](https://www.conrad.de/de/relaiskarte-baustein-conrad-components-197720-12-vdc-24-vdc-197720.html#downloadcenter) over serial interface

## Features

- [x] set specific or all relays on or off
- [x] toggle specific or all relays on or off
- [x] reset (=turn off) specific or all relays
- [ ] run custom command
- [x] multiple cards on one serial device

## Documentation

details about communication protocol are in this [pdf](DOC_8FACH_RELAISKARTE_24V_7A_de_en_fr_nl.pdf)

## Wiring

### One card

- Jumper JP1 in position `1-2`
- connect to USB

### Cascading

- Jumper JP1 in position `2-3` for all cards but the last
- Jumper JP1 at last card in position `1-2`
- connect `GND` terminals with following card
- connect `Txb` of preceeding card with `Rxa` of following card
- connect `Rxb` of preceeding card with `Txa` of following card