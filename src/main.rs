#![no_std]
#![no_main]

mod lsm6dsoxtr;

use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[entry]
unsafe fn main() -> ! {
    let mut sensor = lsm6dsoxtr::init();
    let register_data = lsm6dsoxtr::read_register(&mut sensor, lsm6dsoxtr::RegisterMap::CTRL9_XL);
    defmt::info!("Register data: {:#X}", register_data);
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