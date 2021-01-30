#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use attiny85_hal::clock::*;
use attiny85_hal::delay::Delay;
use attiny85_hal::port::portb::*;
use attiny85_hal::port::*;
use attiny85_hal::prelude::*;

#[attiny85_hal::entry]
fn main() -> ! {
    let dp = attiny85_hal::pac::Peripherals::take().unwrap();
    let mut portb = dp.PORTB.split();
    let mut led0 = portb.pb0.into_output(&mut portb.ddr);
    let mut _led9 = portb.pb1.into_output(&mut portb.ddr);

    let mut x = 0u16;
    loop {
        x = x + 1;
        let mut delay = Delay::<MHz1>::new();

        led0.set_high().void_unwrap();

        delay.delay_ms(1u8);

        led0.set_low().void_unwrap();

        delay.delay_ms(20u8);

        if x == 500 {
            x = 0;
        }
    }
}

fn blink(led: &mut PB0<mode::Output>, x: u16) {}
