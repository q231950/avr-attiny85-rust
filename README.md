
# blink on attiny

## Build

`cargo build`

## Flash

`avrdude -p t85 -c avrispmkII -U flash:w:target/avr-attiny85/debug/avr-attiny85.elf`

## Configure Speed

No matter which speed you select, remember to also adjust the timer to respect the setting:

```rust
let mut delay = Delay::<MHz8>::new();
```

**Factory default 1MHz prescaler**

`avrdude -p t85 -c avrispmkII -U lfuse:w:0x62:m`

**8 MHz**

`avrdude -p t85 -c avrispmkII -U lfuse:w:0xe2:m`

## Chip Interrogation

`-U lfuse:r:-:i`



on      off
10ms    10ms
5ms     5ms
1ms     20ms
1ms     10ms
1ms     5ms
10ms    1ms