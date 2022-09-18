use core::fmt::{self, Display, Formatter, Write};

pub trait Escape {
  /// Write `self` to `f`, escaping if necessary, and appending a newline if
  /// `newline`.
  fn escape(&self, f: &mut Formatter, newline: bool) -> fmt::Result;
}

/// Disable escaping for the wrapped value.
pub struct Trusted<T: Display>(pub T);

impl<T: Display> Escape for T {
  fn escape(&self, f: &mut Formatter, newline: bool) -> fmt::Result {
    if newline {
      writeln!(HtmlEscaper(f), "{}", self)
    } else {
      write!(HtmlEscaper(f), "{}", self)
    }
  }
}

impl<T: Display> Escape for Trusted<T> {
  fn escape(&self, f: &mut Formatter, newline: bool) -> fmt::Result {
    if newline {
      writeln!(f, "{}", self.0)
    } else {
      write!(f, "{}", self.0)
    }
  }
}

/// Escaping wrapper for `core::fmt::Formatter`
pub struct HtmlEscaper<'a, 'b>(pub &'a mut Formatter<'b>);

impl Write for HtmlEscaper<'_, '_> {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    let mut i = 0;
    for (j, c) in s.char_indices() {
      let replacement = match c {
        '"' => Some("&quot;"),
        '&' => Some("&amp;"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '\'' => Some("&apos;"),
        _ => None,
      };
      if let Some(replacement) = replacement {
        if i < j {
          self.0.write_str(&s[i..j])?;
        }
        self.0.write_str(replacement)?;
        i = j + c.len_utf8();
      }
    }

    if i < s.len() {
      self.0.write_str(&s[i..])?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    core::fmt::{self, Display},
  };

  struct Wrapper(&'static str);

  impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      write!(HtmlEscaper(f), "{}", self.0)
    }
  }

  #[test]
  fn unescaped_characters() {
    assert_eq!(Wrapper("hello").to_string(), "hello");
  }

  #[test]
  fn escaped_characters() {
    assert_eq!(Wrapper("\"").to_string(), "&quot;");
    assert_eq!(Wrapper("&").to_string(), "&amp;");
    assert_eq!(Wrapper("'").to_string(), "&apos;");
    assert_eq!(Wrapper("<").to_string(), "&lt;");
    assert_eq!(Wrapper(">").to_string(), "&gt;");
  }

  #[test]
  fn mixed_characters() {
    assert_eq!(Wrapper("foo&bar&baz").to_string(), "foo&amp;bar&amp;baz");
  }
}
