use super::*;

/// Disable escaping for the wrapped value.
pub struct Trusted<T: Display>(pub T);

impl<T: Display> Escape for T {
  fn escape(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(Escaper(f), "{self}")
  }
}

impl<T: Display> Escape for Trusted<T> {
  fn escape(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
