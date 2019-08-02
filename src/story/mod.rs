//! Structures representing a complete `Ink` story.
//!
//! The important internal part of this module is the [`Story`][crate::story::Story]
//! object, which contains an entire story. The user will be interacting with this
//! during runtime.
//!
//! Similar (but not identical) to the [node objects][crate::node] of knots, the story
//! has methods which are run to follow the content. The external syntax is slightly
//! different from the internal following methods, since it is user facing.
//!
//! Most of the rest of this module deals with processing internal data into a form
//! presented to the user, or validating the content of the story as it is being accessed.
//! An example of the latter is the [`Address`][crate::story::Address] object which
//! ensures that an encountered address from an `Ink` file is valid inside of the
//! current story.

mod address;
mod parse;
mod process;
mod story;
mod utils;

pub use address::{Address, ValidateAddresses};
pub use story::{read_story_from_string, Choice, Line, LineBuffer, Prompt, Story};
pub use utils::copy_lines_into_string;
