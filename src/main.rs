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
    let mut led0: PB0<mode::Output> = portb.pb0.into_output(&mut portb.ddr);
    let mut led9: PB1<mode::Output> = portb.pb1.into_output(&mut portb.ddr);
    let mut x = 0u16;

    loop {
        x = x + 1;

        led0.toggle().void_unwrap();
        led9.toggle().void_unwrap();

        let mut delay = Delay::<MHz1>::new();
        delay.delay_ms(x);

        if x == 500 {
            x = 0;
        }

        // for x in 0u16..500u16 {}
    }
}

fn blink(led: &mut PB0<mode::Output>, x: u16) {
    led.toggle().void_unwrap();

    let mut delay = Delay::<MHz1>::new();
    delay.delay_us(5000u16);
}
