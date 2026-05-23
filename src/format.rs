use super::*;

pub trait Format {
  /// Write `self` to `f`, escaping if necessary, indenting continuation lines
  /// by `indent`, and appending a newline if `newline`.
  fn format(&self, f: &mut fmt::Formatter, indent: &'static str, newline: bool) -> fmt::Result;
}
