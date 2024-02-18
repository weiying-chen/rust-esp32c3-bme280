# Rust ESP32-C3 BME280

Example of reading the BME280 sensor on an ESP32-C3 using Rust

## Hardware:

- ESP32-C3 microcontroller
- BME280 sensor
- Breadboard
- Jump wires
- USB-A to Micro-B cable

Step 1: attach the ESP32-C3 to the breadboard (with its headers).

Step 2: attach the BME280 to the breadboard (with its headers).

Step 3: using the jump wires, connect the ESP32-C3 to the BME280: 

- 3V3 to VIN
- GND to GND
- GPIO 2 to SDA
- GPIO 3 to SCL

Note: you can use any available GPIO pins for SDL and SCL, but remember to change the code accordingly.

Step 4: connect the USB cable to your computer or laptop.

## Software:

Step 1: follow these [instructions](https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file#prerequisites) to setup the development environment.

Step 2: Execute `cargo run` on the command line (to build, flash, and monitor). Note: on Linux, you may have to fix a permission [issue](https://github.com/esp-rs/espflash/blob/main/espflash/README.md#permissions-on-linux).

If the universe conspires in your favor, you should see an output like this:

```bash
I (409) rust_esp32_bme280: Successfully initialized BME280 device
I (2529) rust_esp32_bme280: Relative Humidity = 69%
I (2529) rust_esp32_bme280: Temperature = 25°C
I (2529) rust_esp32_bme280: Pressure = 101698 Pa
```

Let me know if I skipped any step.

