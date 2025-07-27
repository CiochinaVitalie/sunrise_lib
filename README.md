# Sunrise Library

This crate provides a driver for Senseair Sunrise sensors. It was
initially developed for `no_std` embedded environments. A new optional
`std` feature allows it to run on Linux as well.

## Building on Linux

Enable the `linux` feature to pull in `linux-embedded-hal` and compile
examples:

```bash
cargo run --example linux --features linux
```

The example assumes the sensor is available on `/dev/i2c-1` and uses
GPIO 27 for the enable pin and GPIO 17 for the `NRDY` pin. Adjust these
values to match your hardware.

