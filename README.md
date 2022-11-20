# xiao-nfr52840-async

xiao-nfr52840-async is a board support package (BSP) library for the Seeed Studio XIAO nRF52840 (Sense).

## Hardware

* (https://wiki.seeedstudio.com/XIAO_BLE/)
* [Schematics] (https://wiki.seeedstudio.com/XIAO-BLE-Sense-Getting-Started/#resources)

## Features

* Uses embassy-nrf HAL for peripherals
* Rust Async/Await


## Program

The module is preprogrammed with a bootloader and the s140 softdevice.

See also:
* (https://github.com/Seeed-Studio/Adafruit_nRF52_Arduino)
* (https://github.com/Seeed-Studio/ArduinoCore-mbed/tree/master/variants/SEEED_XIAO_NRF52840_SENSE/)

[adafruit-nrfutil](https://github.com/adafruit/Adafruit_nRF52_nrfutil) is needed to flash the firmware. It can be istalled with:
```
python3 -m pip install --user adafruit-nrfutil
```

### Building and flashing an example

```
cd examples/led
cargo build --release
cd ../target/thumbv7em-none-eabihf/release
arm-none-eabi-objcopy -O ihex xiao-nfr52840-async-led-example xiao-nfr52840-async-led-example.hex
adafruit-nrfutil dfu genpkg --dev-type 0x0052 --application ./xiao-nfr52840-async-led-example.hex xiao_nfr52840_led_example.zip
adafruit-nrfutil dfu serial -pkg ./xiao_nfr52840_led_example.zip -p /dev/ttyACM0 -b 115200 -fc
```

# License

This work is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
