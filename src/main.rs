#![no_std]
#![no_main]

mod LSM6DSOXTR;

use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[entry]
unsafe fn main() -> ! {
    LSM6DSOXTR::read_accelerometer_who_am_i();
    loop {
        cortex_m::asm::nop();
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