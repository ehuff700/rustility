//! # Rustility
//! rustility is a collection of useful utilities and repeatable logic that I often find myself using in Rust.
//!
//! ## Features
//! **macros** = Enables all macros in this crate.
//!
//! **traits** = Enables all traits in this crate.
//!
//! **full** = Default feature, enables everything.

#[cfg(feature = "macros")]
#[allow(clippy::redundant_closure_call)]
mod macros;

#[cfg(feature = "traits")]
mod traits;

#[cfg(feature = "traits")]
pub use traits::*;
