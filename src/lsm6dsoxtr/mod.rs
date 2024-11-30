
// Pin PB13 is I2C3_SCL
// Pin PB11 is I2C3_SDA

// 1. Enable I2C and clock for the sensor
// 1.1 Enable clock by writing to the I2C3EN bit in the RCC_APB1ENR1 register
// 2. Configure the sensor; do this by writing to CR1, CR2, and I2C_TIMINGR
// 3. Use I2C by pulling the CS line high. This disables SPI and enables I2C
// 4. Set the slave address
// 5. Send a START condition to the slave

use hal::clocks::Clocks;
use defmt;
use defmt_rtt as _; // global logger

use hal::i2c::{I2c, I2cDevice};
use hal::gpio::{Pin, Port, PinMode, OutputType};
use hal::pac;

const LSM6DSOXTR_ADDR: u8 = 0x6B; // I2C address of the accelerometer
const FIFO_CTRL2_REG: u8 = 0x18; // FIFO_CTRL2 register address

pub fn read_accelerometer_who_am_i() {
    defmt::info!("Trying to read WHO_AM_I register of the LSM6DSOXTR accelerometer");
    let mut dp = pac::Peripherals::take().unwrap();
    let clock_cfg = Clocks::default();
    clock_cfg.setup(&mut dp.RCC, &mut dp.FLASH).unwrap();

    // Configure I2C pins
    defmt::info!("Configuring I2C pins");
    let mut scl = Pin::new(Port::B, 13, PinMode::Alt(4));
    scl.output_type(OutputType::OpenDrain);
    let mut sda = Pin::new(Port::B, 11, PinMode::Alt(4));
    sda.output_type(OutputType::OpenDrain);

    // Initialize I2C
    defmt::info!("Initializing I2C");
    let mut i2c = I2c::new(dp.I2C3, I2cDevice::Three, 100_000, &clock_cfg);

    // Read the WHO_AM_I register
    defmt::info!("Reading WHO_AM_I register");
    let mut who_am_i = [0u8; 1];
    defmt::info!("Before reading WHO_AM_I register : {:#X}", who_am_i);
    match i2c.write_read(LSM6DSOXTR_ADDR, &[FIFO_CTRL2_REG], &mut who_am_i) {
        Ok(_) => defmt::info!("WHO_AM_I register read successfully"),
        Err(_) => defmt::info!("Failed to read WHO_AM_I register"),
    }
    defmt::info!("WHO_AM_I register value: {:#X}", who_am_i[0]);

    // Print the result
    hal::debug_workaround();
}