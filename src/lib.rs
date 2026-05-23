#![cfg_attr(not(feature = "std"), no_std)]
//! Spanish language analysis for Pizza search engine.
//!
//! Provides a full-featured Spanish analyzer with light stemming and stop words.
extern crate alloc;
mod stem;
mod stop;

pub mod register;

pub use register::register_all;
pub use stem::SpanishLightStemFilter;
pub use stop::SpanishStopFilter;
