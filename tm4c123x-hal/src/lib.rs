//! HAL for the tm4c123x family of microcontrollers
//!
//! This is an implementation of the [`embedded-hal`] traits for the tm4c123x family of
//! microcontrollers.
//!
//! [`embedded-hal`]: https://github.com/japaric/embedded-hal
//!
//! # Usage
//!
//! To build applications (binary crates) using this crate follow the [cortex-m-quickstart]
//! instructions and add this crate as a dependency in step number 5 and make sure you enable the
//! "rt" Cargo feature of this crate.
//!
//! [cortex-m-quickstart]: https://docs.rs/cortex-m-quickstart/~0.2.3
//!
//! # Examples
//!
//! Examples of *using* these abstractions like these can be found in the documentation of the [`f3`] crate.
//!
//! [`f3`]: https://docs.rs/f3/~0.5.1

//#![deny(missing_docs)]
#![deny(warnings)]
#![allow(deprecated)]
#![no_std]

pub use tm4c_hal::bb;
pub use tm4c_hal::delay;
pub use tm4c_hal::time;

pub mod gpio;
pub mod i2c;
pub mod prelude;
pub mod serial;
pub mod pwm;
pub mod timers;
pub mod spi;
pub mod sysctl;
//pub mod timer;

extern crate embedded_hal as hal;
extern crate nb;
pub use tm4c123x;
pub use tm4c123x::{CorePeripherals, Peripherals};
