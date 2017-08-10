#![feature(associated_type_defaults)]
#![feature(box_syntax)]
// #![warn(missing_docs)]

//! # Delta
//!
//! Delta is a library for writing games
extern crate image;
extern crate rayon;

mod bounds;
mod controller;
mod element;
mod position;
mod sprite;
mod velocity;

pub use bounds::Bounds;
pub use controller::Controller;
pub use element::Element;
pub use position::Position;
pub use sprite::Sprite;
pub use velocity::Velocity;
