use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParserError {
    ParserInternalsInvalidFloatExponent,
}

#[derive(Error, Debug, Diagnostic, PartialEq, Eq)]
#[error("Error parsing float exponent")]
#[diagnostic(
    code(balsapop::parser::invalid_float_exponent),
    url(docsrs),
    help("Float exponents must contain at least one digit (0-9).")
)]
pub(crate) struct InvalidFloatExponent {}
