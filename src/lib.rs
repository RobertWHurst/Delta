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

pub use bounds::Bounds;
pub use controller::Controller;
pub use element::Element;
pub use position::Position;
pub use runtime::Runtime;
pub use scene::Scene;
pub use sprite::Sprite;
pub use velocity::Velocity;
pub use worker::Worker;
