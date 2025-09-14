#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::asm;
use cortex_m_rt::entry;
use rtt_target::rtt_init_defmt;
use stm32f3xx_hal::{
    adc::{Adc, CommonAdc, config::Config},
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    rtt_init_defmt!();
    defmt::info!("Starting light sensor application");

    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut light_pin = gpioa.pa0.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    let mut adc = Adc::new(
        dp.ADC1,
        Config::default(),
        &clocks,
        &CommonAdc::new(dp.ADC1_2, &clocks, &mut rcc.ahb),
    );

    loop {
        let data: u16 = adc.read(&mut light_pin).unwrap();
        // Start conversion and wait until ECO is set.
        defmt::info!("{}", data);

        // Wait 100ms for next conversion
        asm::delay(800_000);
    }
}
