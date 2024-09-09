// This is free and unencumbered software released into the public domain.

#![allow(unused)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
use std as alloc;

pub use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

pub use core::{fmt, result::Result};

#[cfg(feature = "tracing")]
pub use tracing::{debug, error, info, span, warn, Level};
