A crude way of detecting a knock using a piezoelectric sensor, using the rust embedded HAL for STM32.  I'm using a Nucleo64 board, which has headers for STM32, and Arduino.

This uses an analog input, on a 10ms polling cycle, and doesn't debounce or anything like that.  It's pretty much as basic as it's possible to get, but getting it working took a whole lot of time and effort!

Run `openocd`, and then `cargo build && arm-none-eabi-gdb -x openocd.gdb -q target/thumbv7em-none-eabihf/debug/nucleo64-piezoelectric-knock-sensor-rust`.