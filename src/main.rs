#![no_std]
#![no_main]

// This uses an attiny in combination with a CD4026B to drive a 7 segment display.

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use attiny85_hal::clock::*;
use attiny85_hal::delay::Delay;
use attiny85_hal::port::*;
use attiny85_hal::prelude::*;

#[attiny85_hal::entry]
fn main() -> ! {
    let dp = attiny85_hal::pac::Peripherals::take().unwrap();
    let mut portb = dp.PORTB.split();

    let mut bin0 = portb.pb0.into_output(&mut portb.ddr);
    let mut bin1 = portb.pb1.into_output(&mut portb.ddr);
    let mut bin2 = portb.pb2.into_output(&mut portb.ddr);
    let mut bin3 = portb.pb3.into_output(&mut portb.ddr);
    let mut delay = Delay::<MHz1>::new();
    
    bin0.set_low().void_unwrap();
    bin1.set_low().void_unwrap();
    bin2.set_low().void_unwrap();
    bin3.set_low().void_unwrap();

    loop {
        bin0.set_high().void_unwrap();
        bin1.set_low().void_unwrap();
        bin2.set_low().void_unwrap();
        bin3.set_low().void_unwrap();

        delay.delay_ms(250u16);

        bin0.set_low().void_unwrap();
        bin1.set_high().void_unwrap();
        bin2.set_low().void_unwrap();
        bin3.set_low().void_unwrap();

        delay.delay_ms(250u16);

        bin0.set_high().void_unwrap();
        bin1.set_high().void_unwrap();
        bin2.set_low().void_unwrap();
        bin3.set_low().void_unwrap();

        delay.delay_ms(250u16);

        bin0.set_low().void_unwrap();
        bin1.set_low().void_unwrap();
        bin2.set_high().void_unwrap();
        bin3.set_low().void_unwrap();

        delay.delay_ms(250u16);

        bin0.set_high().void_unwrap();
        bin1.set_low().void_unwrap();
        bin2.set_high().void_unwrap();
        bin3.set_low().void_unwrap();

        delay.delay_ms(250u16);

        bin0.set_low().void_unwrap();
        bin1.set_high().void_unwrap();
        bin2.set_high().void_unwrap();
        bin3.set_low().void_unwrap();

        delay.delay_ms(250u16);

        bin0.set_high().void_unwrap();
        bin1.set_high().void_unwrap();
        bin2.set_high().void_unwrap();
        bin3.set_low().void_unwrap();

        delay.delay_ms(250u16);

        bin0.set_low().void_unwrap();
        bin1.set_low().void_unwrap();
        bin2.set_low().void_unwrap();
        bin3.set_high().void_unwrap();

        delay.delay_ms(250u16);

        bin0.set_high().void_unwrap();
        bin1.set_low().void_unwrap();
        bin2.set_low().void_unwrap();
        bin3.set_high().void_unwrap();

        delay.delay_ms(250u16);

        bin0.set_low().void_unwrap();
        bin1.set_low().void_unwrap();
        bin2.set_low().void_unwrap();
        bin3.set_low().void_unwrap();

        delay.delay_ms(250u16);
    }
}
