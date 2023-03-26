#![allow(dead_code, unused_imports, unused_variables)]

use itertools::Itertools;
use rustpython_parser::ast::Location;
use rustpython_parser::Tok;

use ruff_diagnostics::DiagnosticKind;
use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::token_kind::TokenKind;

use crate::rules::pycodestyle::helpers::{is_keyword_token, is_singleton_token};
use crate::rules::pycodestyle::logical_lines::LogicalLineTokens;

#[violation]
pub struct MissingWhitespaceAfterKeyword;

impl Violation for MissingWhitespaceAfterKeyword {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Missing whitespace after keyword")
    }
}

/// E275
#[cfg(feature = "logical_lines")]
pub fn missing_whitespace_after_keyword(
    tokens: LogicalLineTokens,
) -> Vec<(Location, DiagnosticKind)> {
    let mut diagnostics = vec![];

    for (tok0, tok1) in tokens.iter().zip(tokens.iter().skip(1)) {
        let tok0_kind = tok0.kind();
        let tok1_kind = tok1.kind();

        if tok0_kind.is_keyword()
            && !tok0_kind.is_singleton()
            && !matches!(tok0_kind, TokenKind::Async | TokenKind::Await)
            && !(tok0_kind == TokenKind::Except && tok1_kind == TokenKind::Star)
            && !(tok0_kind == TokenKind::Yield && tok1_kind == TokenKind::Rpar)
            && !matches!(tok1_kind, TokenKind::Colon | TokenKind::Newline)
            && tok0.end() == tok1.start()
        {
            diagnostics.push((tok0.end(), MissingWhitespaceAfterKeyword.into()));
        }
    }
    diagnostics
}

#[cfg(not(feature = "logical_lines"))]
pub fn missing_whitespace_after_keyword(
    _tokens: LogicalLineTokens,
) -> Vec<(Location, DiagnosticKind)> {
    vec![]
}
