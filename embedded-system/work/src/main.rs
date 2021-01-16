#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};

#[entry]
fn main() -> ! {
    // let _ = hprintln!("Hello, World!");

    // dp を用いて、GPIO(General purpose i/o) を操作
    if let (Some(dp), Some(cp)) = (stm32::Peripherals::take(), stm32::CorePeripherals::take()) {
        let rcc = dp.RCC.constrain(); // RCC Reset and clock control クロックを供給して回路を有効化
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
        // LED 操作用のインスタンスを取得
        let gpiod = dp.GPIOD.split(); // get D group
        let mut led = gpiod.pd15.into_push_pull_output();

        let mut delay = Delay::new(cp.SYST, clocks); // get CPU 内蔵タイマー

        for _ in 0..5 {
            led.set_high().unwrap(); // 点灯
            delay.delay_ms(100_u32);

            led.set_low().unwrap(); // 消灯
            delay.delay_ms(100_u32);
        }
    }
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
