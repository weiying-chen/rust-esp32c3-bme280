use esp_idf_svc::hal::{
    delay,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};

use bme280::i2c::BME280;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sda = peripherals.pins.gpio2;
    let scl = peripherals.pins.gpio3;
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();
    let mut bme280 = BME280::new_primary(i2c);
    let mut delay = delay::FreeRtos;

    match bme280.init(&mut delay) {
        Ok(_) => log::info!("Successfully initialized BME280 device"),
        Err(_) => log::error!("Failed to initialize BME280 device"),
    }

    loop {
        match bme280.measure(&mut delay) {
            Ok(measurements) => {
                log::info!("Relative Humidity = {:.0}%", measurements.humidity);
                log::info!("Temperature = {:.0}°C", measurements.temperature);
                // log::info!("Pressure = {:,.0} Pa", measurements.pressure);
            }

            Err(e) => log::error!("Failed to get measurements: {:?}", e),
        }

        delay::FreeRtos::delay_ms(1000u32);
    }
}
