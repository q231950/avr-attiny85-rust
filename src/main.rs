#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use attiny85_hal::prelude::*;
use attiny85_hal::delay::Delay;
use attiny85_hal::clock::*;

#[attiny85_hal::entry]
fn main() -> ! {

    let dp = attiny85_hal::pac::Peripherals::take().unwrap();
    let mut delay = Delay::<MHz1>::new();
    let mut portb = dp.PORTB.split();
    let mut led = portb.pb0.into_output(&mut portb.ddr);
    
    loop {
        led.toggle().void_unwrap();
        delay.delay_us(50000u16);
    }
    
}