use rustpython_parser::ast::Location;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ruff_python_ast::types::Range;

use crate::Fix;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DiagnosticKind {
    /// The identifier of the diagnostic, used to align the diagnostic with a rule.
    pub name: String,
    /// The message body to display to the user, to explain the diagnostic.
    pub body: String,
    /// The message to display to the user, to explain the suggested fix.
    pub suggestion: Option<String>,
    /// Whether the diagnostic is automatically fixable.
    pub fixable: bool,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub location: Location,
    pub end_location: Location,
    pub fix: Fix,
    pub parent: Option<Location>,
}

impl Diagnostic {
    pub fn new<T: Into<DiagnosticKind>>(kind: T, range: Range) -> Self {
        Self {
            kind: kind.into(),
            location: range.location,
            end_location: range.end_location,
            fix: Fix::default(),
            parent: None,
        }
    }

    pub fn amend<T: Into<Fix>>(&mut self, edit: T) -> &mut Self {
        self.fix = edit.into();
        self
    }

    pub fn parent(&mut self, parent: Location) -> &mut Self {
        self.parent = Some(parent);
        self
    }
}
