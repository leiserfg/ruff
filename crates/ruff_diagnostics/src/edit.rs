use rustpython_parser::ast::Location;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A text edit to be applied to a source file. Inserts, deletes, or replaces
/// content at a given location.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Edit {
    pub content: String,
    pub location: Location,
    pub end_location: Location,
}

impl Edit {
    pub const fn deletion(start: Location, end: Location) -> Self {
        Self {
            content: String::new(),
            location: start,
            end_location: end,
        }
    }

    pub fn replacement(content: String, start: Location, end: Location) -> Self {
        debug_assert!(!content.is_empty(), "Prefer `Fix::deletion`");

        Self {
            content,
            location: start,
            end_location: end,
        }
    }

    pub fn insertion(content: String, at: Location) -> Self {
        debug_assert!(!content.is_empty(), "Insert content is empty");

        Self {
            content,
            location: at,
            end_location: at,
        }
    }
}
