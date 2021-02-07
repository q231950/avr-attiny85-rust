#![no_std]
#![no_main]

// red + green
// yellow + blue

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

    let mut a1 = portb.pb0.into_output(&mut portb.ddr);
    let mut a2 = portb.pb1.into_output(&mut portb.ddr);
    let mut b1 = portb.pb2.into_output(&mut portb.ddr);
    let mut b2 = portb.pb3.into_output(&mut portb.ddr);
    let mut delay = Delay::<MHz1>::new();

    loop {
        a1.set_high().void_unwrap();
        a2.set_low().void_unwrap();
        b1.set_low().void_unwrap();
        b2.set_low().void_unwrap();

        delay.delay_ms(20u8);

        a1.set_low().void_unwrap();
        a2.set_low().void_unwrap();
        b1.set_high().void_unwrap();
        b2.set_low().void_unwrap();

        delay.delay_ms(20u8);

        a1.set_low().void_unwrap();
        a2.set_high().void_unwrap();
        b1.set_low().void_unwrap();
        b2.set_low().void_unwrap();

        delay.delay_ms(20u8);

        a1.set_low().void_unwrap();
        a2.set_low().void_unwrap();
        b1.set_low().void_unwrap();
        b2.set_high().void_unwrap();

        delay.delay_ms(20u8);
    }
}
