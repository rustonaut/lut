//! lut (lookup tables for every one)
//!
//! Currently this provides a complex macro. In the future this should
//! be a proc-macro which will enable most of the benefits behind this
//! crate with nive ergonomics. Also currently this crate contains some
//! feature-gated lookup tables. They will be moved out of this crate
//! in the future.
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod _impl;
pub use _impl::*;

mod tables;
pub use tables::*;
