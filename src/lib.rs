//! This is a crate providing a common abstraction for I²C port-expanders.  This
//! abstraction is not necessarily the most performant, but it allows using the pins
//! just like direct GPIOs.  Because the pin types also implement the `embedded-hal`
//! digital IO traits, they can also be passed to further drivers downstream (e.g.
//! as a reset or chip-select pin).
//!
//! ## Example
//! ```no_run
//! // Initialize I2C peripheral from HAL
//! let i2c = todo!();
//! # let i2c = embedded_hal_mock::i2c::Mock::new(&[]);
//!
//! // A0: HIGH, A1: LOW, A2: LOW
//! let mut pca9555 = port_expander::Pca9555::new(i2c, true, false, false);
//! let pca_pins = pca9555.split();
//!
//! let io0_0 = pca_pins.io0_0.into_output().unwrap();
//! let io1_5 = pca_pins.io0_1; // default is input
//!
//! io0_0.set_high().unwrap();
//! assert!(io1_5.is_high().unwrap());
//! ```
//!
//! ## Supported Devices
//! The following list is what `port-expander` currently supports.  If you needs
//! support for an additional device, it should be easy to add.  It's best to take
//! a similar existing implementation as inspiration.  Contributions welcome!
//!
//! - [`PCA9536`](Pca9536)
//! - [`PCA9555`](Pca9555)
//! - [`PCF8574A`](Pcf8574a)
//! - [`PCF8574`](Pcf8574)
//!
//! ## Non-local sharing
//! `port-expander` uses the `BusMutex` from
//! [`shared-bus`](https://crates.io/crates/shared-bus) under the hood.  This means
//! you can also make the pins shareable across task/thread boundaries, given that
//! you provide an appropriate mutex type:
//!
//! ```ignore
//! // Initialize I2C peripheral from HAL
//! let i2c = todo!();
//! # let i2c = embedded_hal_mock::i2c::Mock::new(&[]);
//!
//! // A0: HIGH, A1: LOW, A2: LOW
//! let mut pca9555: port_expander::Pca9555<std::sync::Mutex<_>> =
//!     port_expander::Pca9555::with_mutex(i2c, true, false, false);
//! let pca_pins = pca9555.split();
//! ```

#![cfg_attr(not(test), no_std)]

mod bus;
mod common;
pub mod dev;
mod pin;

pub use bus::I2cBus;
pub use common::mode;
pub use pin::Pin;

pub(crate) use bus::I2cExt;
pub(crate) use common::Direction;
pub(crate) use common::PortDriver;
pub(crate) use common::PortDriverTotemPole;

pub use dev::pca9536::Pca9536;
pub use dev::pca9555::Pca9555;
pub use dev::pcf8574::Pcf8574;
pub use dev::pcf8574::Pcf8574a;
