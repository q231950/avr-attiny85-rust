
# blink on attiny

`cargo build`

`avrdude -p t85 -c avrispmkII -U flash:w:target/avr-attiny85/debug/avr-attiny85.elf`

on      off
10ms    10ms
5ms     5ms
1ms     20ms
1ms     10ms
1ms     5ms
10ms    1ms