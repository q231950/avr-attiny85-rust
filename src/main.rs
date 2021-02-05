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
        if animation_clock == 230 {
            animation_clock = 0;
        }
    }

    fn enabled(clock: u16) -> bool {
        match clock % 10 {
            0 => clock <= 25 || clock <= 230 && clock >= 200,
            1 => clock >= 25 && clock <= 50 || clock <= 200 && clock >= 175,
            2 => clock >= 50 && clock <= 75 || clock <= 175 && clock >= 150,
            3 => clock >= 75 && clock <= 100 || clock <= 150 && clock >= 125,
            4 => clock >= 100 && clock <= 125,

            5 => clock >= 100 && clock <= 125,
            6 => clock >= 75 && clock <= 100 || clock <= 150 && clock >= 125,
            7 => clock >= 50 && clock <= 75 || clock <= 175 && clock >= 150,
            8 => clock >= 25 && clock <= 50 || clock <= 200 && clock >= 175,
            9 => clock <= 25 || clock <= 230 && clock >= 200,
            _ => false,
        }
    }
}
