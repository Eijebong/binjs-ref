//! A crate containing tools to manipulate ASTs in generic (i.e. JSON)
//! format.

#![feature(box_patterns)]

extern crate binjs_es6;
extern crate binjs_io;
extern crate binjs_meta;
extern crate binjs_shared;

#[macro_use]
extern crate json;
#[macro_use]
extern crate log;
extern crate rand;

pub mod annotate;

/// Generic instance of `Spec` representing the es6 AST.
pub mod es6;

/// Serializing and deserializing.
pub mod io;

/// Generating random ASTs (for fuzzing purposes).
pub mod pick;

/// Walkers, comparisons.
pub mod syntax;
pub mod util;
