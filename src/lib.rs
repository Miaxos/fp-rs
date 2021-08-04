#![feature(generic_associated_types)]

//! # Functional Programming for Rust
//!  <div align="center">
//!  <!-- CI -->
//!  <img src="https://github.com/Miaxos/fp-rs/actions/workflows/ci.yml/badge.svg" />
//!  <!-- Crates version -->
//!  <a href="https://crates.io/crates/fp-rs">
//!    <img src="https://img.shields.io/crates/v/fp-rs.svg?style=flat-square"
//!    alt="Crates.io version" />
//!  </a>
//!  <!-- Downloads -->
//!  <a href="https://crates.io/crates/fp-rs">
//!    <img src="https://img.shields.io/crates/d/fp-rs.svg?style=flat-square"
//!      alt="Download" />
//!  </a>
//! </div>
//!
//!

mod applicative;
mod apply;
mod chain;
mod functor;
mod hkt;
mod pointed;

trait LendingIterator {
    type Item<'a>;

    fn next<'b>(&'b mut self) -> Self::Item<'b>;
}
