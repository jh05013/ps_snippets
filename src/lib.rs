//! This is a collection of Rust algorithm snippets maintained by
//! jh05013. This is mainly aimed for competitive programming and
//! problem solving.
//! 
//! # Disclaimer
//! While I tried to keep some degree of readability and good API,
//! this crate is made primarily for online judges.
//! This puts the following restrictions:
//! - No external dependencies.
//! - Can't use relatively new features, since some online judges
//!   don't have up-to-date compilers.
//! - Must be easy to copy-paste into the main file, and then
//!   said main file should still be easy to navigate, since most
//!   online judges don't support multiple files.
//! 
//! As such, there are some styling choices different from most crates:
//! - Tabs instead of spaces to save bytes.
//! - Each module is stored in a separate file (easy to copy-paste),
//!   and wrapped with one `mod` (easy to navigate). The mod is then
//!   immediately re-exported so that users don't have to type out the
//!   mod names.
//! - Very short blocks are written in one line to save bytes, e.g.
//!   `if a == b { return None; }`
//! 
//! Furthermore, the API can change at any time and there will be no versioning.
//! Do NOT use this as an external crate.
//! 
//! # Other Rust Snippets
//! - [bamgoeSN](https://bamgoesn.github.io/rust-ps-md/intro.html)
//! - [kiwiyou](https://snippets.kiwiyou.dev/)
//! - [Bubbler](https://github.com/Bubbler-4/rust-problem-solving/tree/main/competitive/src/competelib)

pub mod io;

pub mod ds;
pub mod geometry;
pub mod graph;
pub mod math;
pub mod prime;
pub mod rand;
pub mod string;

mod test;
