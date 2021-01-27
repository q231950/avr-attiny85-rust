
# blink on attiny

`cargo build`

`avrdude -p t85 -c avrispmkII -U flash:w:target/avr-attiny85/debug/avr-attiny85.elf`