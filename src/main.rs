#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::stm32 as device;

#[panic_handler]
fn my_panic_handler(_panic: &PanicInfo) -> ! {
    rprintln!("PANIC: 程序发生严重错误");
    loop {}
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = device::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpio_c = dp.GPIOC.split();
    let mut pc13 = gpio_c.pc13.into_push_pull_output(&mut gpio_c.crh);
    let mut delay = cp.SYST.delay(&clocks);

    rprintln!("=== STM32 RTT 示例程序 ===");
    rprintln!("系统初始化完成，时钟频率：{} Hz", clocks.sysclk().to_Hz());
    rprintln!("开始闪烁 LED...");

    let mut counter: u32 = 0;

    loop {
        pc13.set_high();
        rprintln!("[{}] LED ON", counter);
        delay.delay_ms(1000u32);

        pc13.set_low();
        rprintln!("[{}] LED OFF", counter);
        delay.delay_ms(1000u32);

        counter += 1;

        // 每 10 次循环输出一个统计信息
        if counter % 10 == 0 {
            rprintln!("--- 已运行 {} 个周期 ---", counter);
        }
    }
}
