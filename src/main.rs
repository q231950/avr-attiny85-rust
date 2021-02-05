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
    let mut on: bool;

    pulse.set_high().void_unwrap();
    reset.set_high().void_unwrap();
    reset.set_low().void_unwrap();

    loop {
        match animation_clock % 10 {
            0 => {
                on = animation_clock <= 50;
            }
            1 => {
                on = animation_clock >= 50 && animation_clock <= 100;
            }
            2 => {
                on = animation_clock >= 100 && animation_clock <= 150;
            }
            3 => {
                on = animation_clock >= 150 && animation_clock <= 200;
            }
            4 => {
                on = animation_clock >= 200 && animation_clock <= 250;
            }
            5 => {
                on = animation_clock >= 250 && animation_clock <= 300;
            }
            6 => {
                on = animation_clock >= 300 && animation_clock <= 350;
            }
            7 => {
                on = animation_clock >= 350 && animation_clock <= 400;
            }
            8 => {
                on = animation_clock >= 400 && animation_clock <= 450;
            }
            9 => {
                on = animation_clock >= 450 && animation_clock <= 500;
            }
            _ => {
                on = false;
            }
        }

        if on {
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
        if animation_clock == 500 {
            animation_clock = 0;
        }
    }
}
