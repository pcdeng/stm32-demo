#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::stm32 as device;

#[panic_handler]
fn my_panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let dp = device::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpio_c = dp.GPIOC.split();
    let mut pc13 = gpio_c.pc13.into_push_pull_output(&mut gpio_c.crh);
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        pc13.set_low();
        delay.delay_ms(1000u32);
        pc13.set_high();
        delay.delay_ms(1000u32);
    }
}
