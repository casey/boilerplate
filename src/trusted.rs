use super::*;

/// Disable escaping for the wrapped value.
pub struct Trusted<T: Display>(pub T);

impl<T: Display> Format for T {
  fn format(&self, f: &mut fmt::Formatter, indent: &'static str, newline: bool) -> fmt::Result {
    let mut f = Formatter::new(f, true, indent);
    if newline {
      writeln!(f, "{self}")
    } else {
      write!(f, "{self}")
    }
  }
}

impl<T: Display> Format for Trusted<T> {
  fn format(&self, f: &mut fmt::Formatter, indent: &'static str, newline: bool) -> fmt::Result {
    let mut f = Formatter::new(f, false, indent);
    if newline {
      writeln!(f, "{}", self.0)
    } else {
      write!(f, "{}", self.0)
    }
  }
}
