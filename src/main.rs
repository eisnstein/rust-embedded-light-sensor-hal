#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{self, hprintln};
use stm32f3::stm32f303;

#[entry]
fn main() -> ! {
    // You should see that in your openocd output
    hprintln!("Hello from Discovery");

    let peripherals = &stm32f303::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 8_000_000);
    let rcc = &peripherals.RCC;
    let gpioa = &peripherals.GPIOA;
    let adc1 = &peripherals.ADC1;

    // Configure Port A Pin 0

    // Set HSI clock on
    rcc.cr.write(|w| w.hsion().on());
    // Enable GPIO Port A clock
    rcc.ahbenr.write(|w| w.iopaen().enabled());
    // Enable ADC1 clock
    rcc.ahbenr.write(|w| w.adc12en().enabled());
    // Enable SYSCFG clock
    // rcc.apb2enr.write(|w| w.syscfgen().enabled());

    // Set Pin 0 to analog input
    gpioa.moder.write(|w| w.moder0().analog());

    // Configure ADC1

    // Set single conversion mode
    adc1.cfgr.write(|w| w.cont().single());
    // 8bit data resolution
    adc1.cfgr.write(|w| w.res().bits8());

    // Reset control register
    adc1.cr.reset();

    hprintln!("Enable vrs");
    // Enable voltage regulation sequence.
    // This has to be done before the calibration.
    adc1.cr.write(|w| w.advregen().intermediate());
    adc1.cr.write(|w| w.advregen().enabled());

    // Wait for the startup time of the ADC voltage regulator
    // see STM32f303 reference manual section 15.3.6
    delay.delay_us(10);

    hprintln!("Start calibration");
    // Start calibration
    // adc1.cr.write(|w| w.adcaldif().single_ended());
    // adc1.cr.write(|w| w.adcal().calibration());
    // while adc1.cr.read().adcal().bit_is_set() {}

    // Wait for the calibration to be finished.
    // see STM32f303 reference manual section 15.3.6
    // delay.delay_us(10);

    hprintln!("Enable adc");
    // Enable the ADC
    adc1.cr.write(|w| w.aden().enabled());
    while adc1.isr.read().adrdy().is_not_ready() {}

    // Set data resolution to 8bit
    adc1.cfgr.write(|w| w.res().bits8());
    // Select channel 1
    adc1.sqr1.write(|w| unsafe { w.sq1().bits(1) });
    // Set sample time
    adc1.smpr1.write(|w| w.smp1().cycles601_5());

    hprintln!("Start loop");
    loop {
        // Start conversion and wait until ECO is set.
        adc1.cr.write(|w| w.adstart().start_conversion());
        while adc1.isr.read().eoc().is_not_complete() {}

        // Read data from data register
        let data = adc1.dr.read().rdata().bits();

        hprintln!("{}", data);

        // Wait 100ms for next conversion
        delay.delay_ms(100);
    }
}
