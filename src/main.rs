#![no_std]
#![no_main]
use cortex_m_semihosting::hprintln;
// pick a panicking behavior
use crate::hal::delay::Delay;
use crate::hal::{
    adc::{config::AdcConfig, config::SampleTime, Adc},
    pac,
    prelude::*,
};
use cortex_m_rt::entry;
use panic_halt as _;
//use stm32f4::stm32f401::{interrupt, Interrupt, NVIC};
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let gpioa = dp.GPIOA.split();
        let pa0 = gpioa.pa0.into_analog();
        let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let mut delay = Delay::new(cp.SYST, &clocks);

        loop {
            let sample = adc.convert(&pa0, SampleTime::Cycles_480);
            if (sample as f64 / 4096.) * 5. > 1. {
                hprintln!("Knock!").unwrap();
            }
            delay.delay_ms(10_u32);
        }
    }
    loop {}
}
