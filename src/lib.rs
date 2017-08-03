#![feature(associated_type_defaults)]
#![warn(missing_docs)]

//! # Delta
//!
//! Delta is a library for writing games

pub extern crate image;
pub extern crate time;

pub mod components;
pub mod entities;
pub mod sprites;

mod engine;

pub use engine::*;
