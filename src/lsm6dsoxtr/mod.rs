use hal::clocks::Clocks;
use defmt;
use defmt_rtt as _; // global logger

use hal::i2c::{I2c, I2cDevice};
use hal::gpio::{Pin, Port, PinMode, OutputType};
use hal::pac;

#[derive(Debug)]
pub enum RegisterMap {
    CHIP_I2C_ADDR,
    WHO_AM_I,
    CTRL9_XL,
}

impl RegisterMap {
    fn address(&self) -> u8 {
        match self {
            RegisterMap::CHIP_I2C_ADDR => 0x6B,
            RegisterMap::WHO_AM_I => 0x0F,
            RegisterMap::CTRL9_XL => 0x18,
        }
    }
}

pub fn init() -> I2c<pac::I2C3> {
    defmt::info!("Initializing I2C");

    // Setup the clock
    defmt::info!("Enabling I2C3 clock...");
    let mut dp = pac::Peripherals::take().unwrap();
    let clock_cfg = Clocks::default();
    clock_cfg.setup(&mut dp.RCC, &mut dp.FLASH).unwrap();

    // Configure I2C3 GPIO pins
    defmt::info!("Configuring I2C3 GPIO pins...");
    let mut scl = Pin::new(Port::B, 13, PinMode::Alt(4));
    scl.output_type(OutputType::OpenDrain);
    let mut sda = Pin::new(Port::B, 11, PinMode::Alt(4));
    sda.output_type(OutputType::OpenDrain);

    // Initialize I2C
    return I2c::<pac::I2C3>::new(dp.I2C3, I2cDevice::Three, 100_000, &clock_cfg);
}

pub fn read_register(i2c: &mut I2c<pac::I2C3>, register: RegisterMap) -> u8 {
    let mut data = [0u8; 1];
    match i2c.write_read(RegisterMap::CHIP_I2C_ADDR.address(), &[register.address()], &mut data) {
        Ok(_) => defmt::info!("Read register {:#X} successfully", register.address()),
        Err(_) => defmt::info!("Failed to read register {:#X}", register.address()),
    }
    return data[0];
}