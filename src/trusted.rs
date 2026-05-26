use super::*;

/// Disable escaping for the wrapped value.
pub struct Trusted<T: Display>(pub T);

impl<T: Display> Format for T {
  fn format(&self, f: &mut fmt::Formatter, indent: &'static str, trim: bool) -> fmt::Result {
    let mut f = Formatter::new(f, true, indent, trim);
    write!(f, "{self}")
  }
}

impl<T: Display> Format for Trusted<T> {
  fn format(&self, f: &mut fmt::Formatter, indent: &'static str, trim: bool) -> fmt::Result {
    let mut f = Formatter::new(f, false, indent, trim);
    write!(f, "{}", self.0)
  }
}
