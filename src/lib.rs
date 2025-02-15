//! [![sponsor-us]](https://github.com/sponsors/LechevSpace)&ensp;[![github]](https://github.com/LechevSpace/embedded-canvas)&ensp;[![crates-io]](https://crates.io/crates/embedded-canvas)
//!
//! `embedded-canvas` is a convenient crate for [`embedded-graphics`]
//! and provides a [`Canvas`](#canvas) and [`CanvasAt`](#CanvasAt) on which you
//! can draw anything with ease before drawing the pixels on the embedded display.
//!
//! > canvas - a piece of cloth backed or framed as a surface for a painting
//!
//! Based on [`embedded-graphics-core`] and [`embedded-graphics`]
//! (see `transform` feature in [Crate features](#crate-features)).
//!
//! This crate is `no_std` but requires `alloc` for the time being.
//!
//! The main advantages of the canvases in this crate are:
//!
//! 1. **Transparency** - pixels that haven't been drawn, won't override pixels on the display.
//! 2. **Cropping** - The ability to crop leaves only the part of the canvas you want to
//! draw on the display. This is especially useful when you want to
//! partially show text, figures and images.
//!
//! [`embedded-graphics`]: https://crates.io/crates/embedded-graphics
//! [`embedded-graphics-core`]: https://crates.io/crates/embedded-graphics-core
//!
//! # How to work with canvases
//!
//! There are **two** main canvases you can work with:
//!
//! ## `Canvas`
//!
//! A [`Canvas`] which you can draw on with origin [`Point::zero()`](embedded_graphics_core::geometry::Point::zero).
//! The canvas location is not set for the provided display.
//!
//! After drawing decide where to place it on the display using the methods:
//! - [`Canvas::place_at(top_left: Point) -> CanvasAt`](Canvas::place_at)
//! - [`Canvas::place_center(center: Point) -> CanvasAt`](Canvas::place_center)
//!
//! ## `CanvasAt`
//!
//! [`CanvasAt`] is a type of canvas ready to be drawn on the display
//! at specified location (hence the name [`CanvasAt`]).
//!
//! There are two ways of using [`CanvasAt`]:
//!
//! 1. Directly placing the [`CanvasAt`] on specified location on the display and drawing inside.
//! 2. Create a [`Canvas`] and when ready to draw it on the display place the
//!  [`Canvas`] at specified location using the methods:
//!   - [`Canvas::place_at(top_left: Point) -> CanvasAt`](Canvas::place_at)
//!   - [`Canvas::place_center(center: Point) -> CanvasAt`](Canvas::place_center)
//!
//! # Crate features
//! - `default` features - `transform`
//! - `transform` - enables the trait implementation of [`embedded_graphics::transform::Transform`] for [`CanvasAt`].
//!
//! [github]: https://img.shields.io/badge/github-3873AD?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/crates/v/embedded-canvas?logo=rust&style=for-the-badge
//! [sponsor-us]: https://img.shields.io/github/sponsors/LechevSpace?color=bf3989&label=Sponsor%20us&style=for-the-badge&logoColor=bf3989&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyBoZWlnaHQ9IjE2IiB2aWV3Qm94PSIwIDAgMTYgMTYiIHZlcnNpb249IjEuMSIgd2lkdGg9IjE2IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogICAgPHBhdGggZmlsbD0iI2JmMzk4OSIgZmlsbC1ydWxlPSJldmVub2RkIiBkPSJNNC4yNSAyLjVjLTEuMzM2IDAtMi43NSAxLjE2NC0yLjc1IDMgMCAyLjE1IDEuNTggNC4xNDQgMy4zNjUgNS42ODJBMjAuNTY1IDIwLjU2NSAwIDAwOCAxMy4zOTNhMjAuNTYxIDIwLjU2MSAwIDAwMy4xMzUtMi4yMTFDMTIuOTIgOS42NDQgMTQuNSA3LjY1IDE0LjUgNS41YzAtMS44MzYtMS40MTQtMy0yLjc1LTMtMS4zNzMgMC0yLjYwOS45ODYtMy4wMjkgMi40NTZhLjc1Ljc1IDAgMDEtMS40NDIgMEM2Ljg1OSAzLjQ4NiA1LjYyMyAyLjUgNC4yNSAyLjV6TTggMTQuMjVsLS4zNDUuNjY2LS4wMDItLjAwMS0uMDA2LS4wMDMtLjAxOC0uMDFhNy42NDMgNy42NDMgMCAwMS0uMzEtLjE3IDIyLjA3NSAyMi4wNzUgMCAwMS0zLjQzNC0yLjQxNEMyLjA0NSAxMC43MzEgMCA4LjM1IDAgNS41IDAgMi44MzYgMi4wODYgMSA0LjI1IDEgNS43OTcgMSA3LjE1MyAxLjgwMiA4IDMuMDIgOC44NDcgMS44MDIgMTAuMjAzIDEgMTEuNzUgMSAxMy45MTQgMSAxNiAyLjgzNiAxNiA1LjVjMCAyLjg1LTIuMDQ1IDUuMjMxLTMuODg1IDYuODE4YTIyLjA4IDIyLjA4IDAgMDEtMy43NDQgMi41ODRsLS4wMTguMDEtLjAwNi4wMDNoLS4wMDJMOCAxNC4yNXptMCAwbC4zNDUuNjY2YS43NTIuNzUyIDAgMDEtLjY5IDBMOCAxNC4yNXoiPjwvcGF0aD4KPC9zdmc%2BCg%3D%3D
#![no_std]

#[doc(inline)]
pub use canvas::{Canvas, CanvasAt};

mod canvas;
