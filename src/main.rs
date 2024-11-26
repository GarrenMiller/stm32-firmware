#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use hal::{
    self,
    clocks::Clocks,
    gpio::{Pin, PinMode, Port},
    pac,
};

// use defmt_rtt as _; // global logger


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();

    let clock_cfg = Clocks::default();

    clock_cfg.setup(&mut dp.RCC, &mut dp.FLASH).unwrap();

    let mut dealy = Delay::new(cp.SYST, clock_cfg.systick());
    
    // TODO: This does not work; have to use the SPI interface
    let mut led = Pin::new(Port::C, 13, PinMode::Output);

    loop {
        led.set_low();
        dealy.delay_ms(1000);
        led.set_high();
        dealy.delay_ms(1000);
    }
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}