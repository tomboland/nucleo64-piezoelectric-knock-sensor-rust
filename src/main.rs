#![no_std]
#![no_main]
use crate::hal::delay::Delay;
use crate::hal::{
    adc::{config::AdcConfig, config::Resolution, config::SampleTime, Adc},
    pac,
    prelude::*,
};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f4xx_hal as hal;

const ADC_RESOLUTION: Resolution = Resolution::Twelve;
const ADC_RESOLUTION_BITS: u32 = 12;
const REFERENCE_VOLTAGE: f64 = 3.3;
const KNOCK_THRESHOLD_VOLTAGE: f64 = 1.;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let gpioa = dp.GPIOA.split();
        let pa0 = gpioa.pa0.into_analog();
        let mut adc = Adc::adc1(
            dp.ADC1,
            true,
            AdcConfig::default().resolution(ADC_RESOLUTION),
        );

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let mut delay = Delay::new(cp.SYST, &clocks);

        loop {
            let sample = adc.convert(&pa0, SampleTime::Cycles_480);
            if (sample as f64 / (2 as u32).pow(ADC_RESOLUTION_BITS) as f64) * REFERENCE_VOLTAGE
                > KNOCK_THRESHOLD_VOLTAGE
            {
                hprintln!("Knock!").unwrap();
            }
            delay.delay_ms(10_u32);
        }
    }
    loop {}
}
