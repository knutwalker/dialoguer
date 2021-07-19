#![allow(clippy::needless_doctest_main)]

#[cfg(feature = "confirm")]
pub mod confirm;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "multi_select")]
pub mod multi_select;
#[cfg(feature = "password")]
pub mod password;
#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "sort")]
pub mod sort;
