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

    let mut a = portb.pb0.into_output(&mut portb.ddr);
    let mut b = portb.pb1.into_output(&mut portb.ddr);
    let mut c = portb.pb2.into_output(&mut portb.ddr);
    let mut d = portb.pb3.into_output(&mut portb.ddr);
    let mut led = portb.pb4.into_output(&mut portb.ddr);
    let mut delay = Delay::<MHz8>::new();

    // let mut animation_clock = 0u16;
    let mut index = 0u8;

    loop {
        if index > 9 {
            index = 0;
        }

        led.set_low().void_unwrap();

        enable_led_at_index(index, &mut a, &mut b, &mut c, &mut d);

        led.set_high().void_unwrap();
        // update(&mut led, index, animation_clock);

        delay.delay_ms(2u8);

        // if animation_clock == 500 {
        //     animation_clock = 0;
        // }

        index = index + 1;
        // animation_clock = animation_clock + 1;
    }
}

// fn update(led: &mut PB4<mode::Output>, _index: u8, _animation_clock: u16) {
//     led.set_high().void_unwrap();
// }

fn enable_led_at_index(
    index: u8,
    a: &mut PB0<mode::Output>,
    b: &mut PB1<mode::Output>,
    c: &mut PB2<mode::Output>,
    d: &mut PB3<mode::Output>,
) {
    match index {
        0 => {
            a.set_low().void_unwrap();
            b.set_low().void_unwrap();
            c.set_low().void_unwrap();
            d.set_low().void_unwrap();
        }
        1 => {
            a.set_high().void_unwrap();
            // b.set_low().void_unwrap();
            // c.set_low().void_unwrap();
            // d.set_low().void_unwrap();
        }
        2 => {
            a.set_low().void_unwrap();
            b.set_high().void_unwrap();
            // c.set_low().void_unwrap();
            // d.set_low().void_unwrap();
        }
        3 => {
            a.set_high().void_unwrap();
            // b.set_high().void_unwrap();
            // c.set_low().void_unwrap();
            // d.set_low().void_unwrap();
        }
        4 => {
            a.set_low().void_unwrap();
            b.set_low().void_unwrap();
            c.set_high().void_unwrap();
            // d.set_low().void_unwrap();
        }
        5 => {
            a.set_high().void_unwrap();
            // b.set_low().void_unwrap();
            // c.set_high().void_unwrap();
            // d.set_low().void_unwrap();
        }
        6 => {
            a.set_low().void_unwrap();
            b.set_high().void_unwrap();
            // c.set_high().void_unwrap();
            // d.set_low().void_unwrap();
        }
        7 => {
            a.set_high().void_unwrap();
            // b.set_high().void_unwrap();
            // c.set_high().void_unwrap();
            // d.set_low().void_unwrap();
        }
        8 => {
            a.set_low().void_unwrap();
            b.set_low().void_unwrap();
            c.set_low().void_unwrap();
            d.set_high().void_unwrap();
        }
        9 => {
            a.set_high().void_unwrap();
            // b.set_low().void_unwrap();
            // c.set_low().void_unwrap();
            d.set_high().void_unwrap();
        }
        _ => (),
    }
}
