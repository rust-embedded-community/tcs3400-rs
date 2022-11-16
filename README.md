# Rust TCS3400 RGB Color Light to Digital Converter with IR Filter Driver

[![crates.io](https://img.shields.io/crates/v/tcs3400.svg)](https://crates.io/crates/tcs3400)
[![Docs](https://docs.rs/tcs3400/badge.svg)](https://docs.rs/tcs3400)
[![Build Status](https://github.com/andresv/tcs3400-rs/workflows/Build/badge.svg)](https://github.com/andresv/tcs3400-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/andresv/tcs3400-rs/badge.svg?branch=master)](https://coveralls.io/github/andresv/tcs3400-rs?branch=master)

This is a platform agnostic Rust driver for the TCS3400 RGB color light to
digital converter with IR filter, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Enable/disable the device.
- Enable/disable the RGB converter.
- Set RGB converter gain.
- Enable/disable the RGB converter interrupt generation.
- Set the RGB converter interrupt clear channel low/high thresholds.
- Set the RGB converter interrupt persistence.
- Set the number of integration cycles.
- Enable/disable the wait feature.
- Set the number of wait time cycles.
- Enable/disable the *wait long* setting.
- Read status of RGB converter.
- Read the clear (unfiltered) channel measurement.
- Read the red channel measurement.
- Read the green channel measurement.
- Read the blue channel measurement.
- Read the measurement of all channels at once.
- Read the device ID.

## The device
The TCS3400 device provides a digital return of red, green, blue (RGB), and
clear light sensing values. An IR blocking filter, integrated on-chip and
localized to the color sensing photodiodes, minimizes the IR spectral
component of the incoming light and allows color measurements to be made
accurately. The high sensitivity, wide dynamic range, and IR blocking
filter make the TCS3400 an ideal color sensor solution for use under
varying lighting conditions and through attenuating materials.

The TCS3400 color sensor has a wide range of applications including RGB LED
backlight control, solid-state lighting, health/fitness products,
industrial process controls and medical diagnostic equipment. In addition,
the IR blocking filter enables the TCS3400 to perform ambient light sensing
(ALS). Ambient light sensing is widely used in display-based products such
as cell phones, notebooks, and TVs to sense the lighting environment and
enable automatic display brightness for optimal viewing and power savings.
The TCS3400, itself, can enter a lower-power wait state between light
sensing measurements to further reduce the average power consumption.

Datasheet:
- [TCS3400](https://ams.com/documents/20143/36005/TCS3400_DS000411_5-00.pdf)

This driver is compatible with the devices TCS34005 and TCS34007.

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

```rust
use linux_embedded_hal::I2cdev;
use tcs3400::Tcs3400;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Tcs3400::new(dev);
    sensor.enable().unwrap();
    sensor.enable_rgbc().unwrap();
    while !sensor.is_rgbc_status_valid().unwrap() {
        // wait for measurement to be available
    }
    let m = sensor.read_all_channels().unwrap();
    println!(
        "Measurements: clear = {}, red = {}, green = {}, blue = {}",
        m.clear, m.red, m.green, m.blue
    );
}
```

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.31.0 and up. It *might*
compile with older versions but that may change in any new patch release.

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/andresv/tcs3400-rs/issues).

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

