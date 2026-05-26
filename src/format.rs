use super::*;

pub trait Format {
  /// Write `self` to `f`, escaping if necessary, indenting continuation lines
  /// by `indent`, and dropping a single trailing newline from the output if
  /// `trim` is true.
  fn format(&self, f: &mut fmt::Formatter, indent: &'static str, trim: bool) -> fmt::Result;
}
