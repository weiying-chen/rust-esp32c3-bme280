# Rust ESP32-C3 BME280

Example of reading the BME280 sensor on an ESP32-C3 using Rust

## Hardware:

- ESP32-C3 microcontroller
- BME280 sensor
- Breadboard
- Jump wires
- USB-A to Micro-B cable

Step 1: Attach the ESP32-C3 to the breadboard (with its headers).

Step 2: Attach the BME280 to the breadboard (with its headers).

Step 3: Using the jump wires, connect the ESP32-C3 to the BME280: 

- VIN to 3V3
- GND to GND
- SDL to GPIO 2
- SCL to GPIO 3

Note: You can use any available GPIO pins for SDL and SCL.

Step 4: Connect the USB cable to your computer or laptop.

## Software:

Step 1: follow these [instructions](https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file#prerequisites) to setup the development environment.

Step 2: execute `cargo run` in the terminal (to build, flash, and monitor). Note: on Linux, you may have to [fix](https://github.com/esp-rs/espflash/blob/main/espflash/README.md#permissions-on-linux) a permission issue.

If the universe conspires in your favor, you should see an output like this:

```bash
I (409) rust_esp32_bme280: Successfully initialized BME280 device
I (2529) rust_esp32_bme280: Relative Humidity = 69%
I (2529) rust_esp32_bme280: Temperature = 25°C
I (2529) rust_esp32_bme280: Pressure = 101698 Pa
```

Let me know if I skipped any step.

