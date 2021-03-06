//! Validate story and variable names, addresses, expressions, and conditions.

pub(self) mod namespace;
pub(self) mod validate;

pub use validate::{validate_story_content, KnotValidationInfo, ValidateContent, ValidationData};
