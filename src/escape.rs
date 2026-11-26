use super::*;

pub trait Escape {
  /// Write `self` to `f`, escaping if necessary.
  fn escape(&self, f: &mut fmt::Formatter) -> fmt::Result;
}
