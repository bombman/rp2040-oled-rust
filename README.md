# Hello Rust on SSD1306 OLED Display

This project demonstrates how to display "Hello Rust" on an SSD1306 OLED display using a Raspberry Pi Pico and the Rust programming language. The display is centered on the screen using the `embedded_graphics` crate.

## Hardware Requirements

- Raspberry Pi Pico
- SSD1306 OLED display (128x64 pixels)
- I2C connection between the Pico and the display
  - SDA connected to GPIO4
  - SCL connected to GPIO5

## Software Requirements

- Rust toolchain (nightly)
- `cargo-generate` installed
- The following Rust crates:
  - `cortex-m-rt`
  - `embedded-graphics`
  - `fugit`
  - `panic-halt`
  - `rp2040-hal`
  - `ssd1306`

## Wiring

Connect the Raspberry Pi Pico to the SSD1306 display as follows:

- GPIO4 (Pico) to SDA (Display)
- GPIO5 (Pico) to SCL (Display)
- GND (Pico) to GND (Display)
- VBUS (Pico) to VCC (Display)

## Building and Running

1. Ensure you have the Rust toolchain installed. If not, install it using [rustup](https://rustup.rs/).
2. Add the nightly toolchain and target for ARM Cortex-M0+:
   ```sh
   rustup toolchain install nightly
   rustup target add thumbv6m-none-eabi
   ```
Alternatively, to run the project directly on the Raspberry Pi Pico using cargo run, use the following command:  
     ```sh
    cargo run --bin oled
    ```
