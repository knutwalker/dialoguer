//! dialoguer is a library for Rust that helps you build useful small
//! interactive user inputs for the command line.  It provides utilities
//! to render various simple dialogs like confirmation prompts, text
//! inputs and more.
//!
//! Best paired with other libraries in the family:
//!
//! * [indicatif](https://docs.rs/indicatif)
//! * [console](https://docs.rs/console)
//!
//! # Crate Contents
//!
//! * Confirmation prompts
//! * Input prompts (regular and password)
//! * Input validation
//! * Selections prompts (single and multi)
//! * Other kind of prompts
//! * Editor launching

pub use console;
#[cfg(feature = "editor")]
pub use edit::Editor;
#[cfg(feature = "confirm")]
pub use prompts::confirm::Confirm;
#[cfg(feature = "input")]
pub use prompts::input::Input;
#[cfg(feature = "multi_select")]
pub use prompts::multi_select::MultiSelect;
#[cfg(feature = "password")]
pub use prompts::password::Password;
#[cfg(feature = "select")]
pub use prompts::select::Select;
#[cfg(feature = "sort")]
pub use prompts::sort::Sort;
pub use validate::Validator;

#[cfg(feature = "editor")]
mod edit;
mod prompts;
pub mod theme;
mod validate;
