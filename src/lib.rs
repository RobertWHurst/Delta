#![feature(associated_type_defaults)]
#![feature(box_syntax)]
// #![warn(missing_docs)]

//! # Delta
//!
//! Delta is a library for writing games
extern crate image;
extern crate num_cpus;
extern crate rayon;
extern crate time;

mod bounds;
mod controller;
mod element;
mod position;
mod runtime;
mod scene;
mod sprite;
mod velocity;
mod worker;

pub use bounds::*;
pub use controller::*;
pub use element::*;
pub use position::*;
pub use runtime::*;
pub use scene::*;
pub use sprite::*;
pub use velocity::*;
pub use worker::*;
