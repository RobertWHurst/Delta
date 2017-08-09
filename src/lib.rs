#![feature(associated_type_defaults)]
// #![warn(missing_docs)]

//! # Delta
//!
//! Delta is a library for writing games
extern crate image;
extern crate rayon;

mod bounds;
mod element;
mod sprite;

pub use element::Element;
pub use sprite::Sprite;
