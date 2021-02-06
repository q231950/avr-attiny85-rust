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
    let mut animation_clock = 25i16;
    let mut index = 0u8;
    pulse.set_high().void_unwrap();
    reset.set_high().void_unwrap();
    reset.set_low().void_unwrap();
    let mut direction = 1i16;

    loop {
        if enabled(index, animation_clock) {
            led.set_high().void_unwrap();
        }

        delay.delay_us(500u16);

        led.set_low().void_unwrap();

        index += 1;
        if index == 10 {
            reset.set_high().void_unwrap();
            reset.set_low().void_unwrap();
            index = 0;
        } else {
            pulse.set_low().void_unwrap();
            pulse.set_high().void_unwrap();
        }

        animation_clock += direction;
        // The clock goes back and forth in time
        if animation_clock == 500 || animation_clock == 0 {
            direction = -direction;
        }
    }

    fn enabled(index: u8, clock: i16) -> bool {
        return match index {
            0 => clock <= 50,
            1 => clock >= 50 && clock <= 100,
            2 => clock >= 100 && clock <= 150,
            3 => clock >= 150 && clock <= 200,
            4 => clock >= 200 && clock <= 250,
            5 => clock >= 250 && clock <= 300,
            6 => clock >= 300 && clock <= 350,
            7 => clock >= 350 && clock <= 400,
            8 => clock >= 400 && clock <= 450,
            9 => clock >= 450 && clock <= 500,
            _ => false,
        };
    }
}
