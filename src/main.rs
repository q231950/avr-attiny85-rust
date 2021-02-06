#![no_std]
#![no_main]

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

    let mut pulse = portb.pb0.into_output(&mut portb.ddr);
    let mut reset = portb.pb1.into_output(&mut portb.ddr);
    let mut led = portb.pb4.into_output(&mut portb.ddr);
    let mut delay = Delay::<MHz8>::new();
    let mut animation_clock = 0u16;
    pulse.set_high().void_unwrap();
    reset.set_high().void_unwrap();
    reset.set_low().void_unwrap();

    loop {
        if enabled(animation_clock) {
            led.set_high().void_unwrap();
        }

        delay.delay_ms(2u8);

        led.set_low().void_unwrap();

        if animation_clock % 10 == 9 {
            reset.set_high().void_unwrap();
            reset.set_low().void_unwrap();
        } else {
            pulse.set_low().void_unwrap();
            pulse.set_high().void_unwrap();
        }

        animation_clock += 1;
        if animation_clock == 410 {
            animation_clock = 0;
        }
    }

    fn enabled(animation_clock: u16) -> bool {
        match animation_clock % 10 {
            0 => animation_clock <= 10,
            1 => animation_clock >= 10 && animation_clock <= 25,
            2 => animation_clock >= 25 && animation_clock <= 40,
            3 => animation_clock >= 40 && animation_clock <= 100,
            4 => animation_clock >= 100 && animation_clock <= 156,
            5 => animation_clock >= 156 && animation_clock <= 225,
            6 => animation_clock >= 225 && animation_clock <= 325,
            7 => animation_clock >= 325 && animation_clock <= 385,
            8 => animation_clock >= 385 && animation_clock <= 400,
            9 => animation_clock >= 400 && animation_clock <= 410,
            _ => false,
        }
    }
}
