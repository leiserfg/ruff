use rustpython_parser::ast::Location;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::edit::Edit;

/// A collection of edits to be applied to a source file.
#[derive(Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Fix {
    edits: Vec<Edit>,
}

impl Fix {
    pub fn new(edits: Vec<Edit>) -> Self {
        // TODO(charlie): Ensure that the edits are sorted and non-overlapping.
        debug_assert!(!edits.is_empty(), "Fix must have at least one edit");

        Self { edits }
    }

    pub fn none() -> Self {
        Self::default()
    }

    pub fn is_none(&self) -> bool {
        self.edits.is_empty()
    }

    pub fn location(&self) -> Location {
        self.edits[0].location
    }

    pub fn edits(&self) -> &[Edit] {
        &self.edits
    }
}

impl From<Edit> for Fix {
    fn from(edit: Edit) -> Self {
        Self { edits: vec![edit] }
    }
}
