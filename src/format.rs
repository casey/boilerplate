use core::fmt::{self, Display, Write};

pub trait Format {
  /// Write `self` to `f`, escaping if necessary, indenting continuation lines
  /// by `indent`, and appending a newline if `newline`.
  fn format(&self, f: &mut fmt::Formatter, indent: &'static str, newline: bool) -> fmt::Result;
}

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

/// A `core::fmt::Write` adapter that optionally HTML-escapes its input and
/// optionally prepends `indent` after every internal `\n`.
///
/// Indent emission is lazy: when a `\n` is written, the indent is queued and
/// only emitted before the next non-`\n` content. This means a trailing `\n`
/// in the input does not produce a trailing indent.
pub struct Formatter<'a, 'b> {
  inner: &'a mut fmt::Formatter<'b>,
  escape: bool,
  indent: &'static str,
  pending_indent: bool,
}

impl<'a, 'b> Formatter<'a, 'b> {
  pub fn new(inner: &'a mut fmt::Formatter<'b>, escape: bool, indent: &'static str) -> Self {
    Self {
      inner,
      escape,
      indent,
      pending_indent: false,
    }
  }

  fn flush_pending_indent(&mut self) -> fmt::Result {
    if self.pending_indent {
      self.inner.write_str(self.indent)?;
      self.pending_indent = false;
    }
    Ok(())
  }
}

impl Write for Formatter<'_, '_> {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    let mut chunk_start = 0;

    for (i, &byte) in s.as_bytes().iter().enumerate() {
      let replacement = if self.escape {
        match byte {
          b'"' => Some("&quot;"),
          b'&' => Some("&amp;"),
          b'<' => Some("&lt;"),
          b'>' => Some("&gt;"),
          b'\'' => Some("&apos;"),
          _ => None,
        }
      } else {
        None
      };

      if byte != b'\n' && replacement.is_none() {
        continue;
      }

      if chunk_start < i {
        self.flush_pending_indent()?;
        self.inner.write_str(&s[chunk_start..i])?;
      }

      if let Some(replacement) = replacement {
        self.flush_pending_indent()?;
        self.inner.write_str(replacement)?;
      } else {
        self.inner.write_str("\n")?;
        if !self.indent.is_empty() {
          self.pending_indent = true;
        }
      }

      chunk_start = i + 1;
    }

    if chunk_start < s.len() {
      self.flush_pending_indent()?;
      self.inner.write_str(&s[chunk_start..])?;
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

  #[cfg(not(feature = "reload"))]
  use alloc::string::ToString;

  struct Wrapper {
    escape: bool,
    indent: &'static str,
    value: &'static str,
  }

  impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let mut f = Formatter::new(f, self.escape, self.indent);
      f.write_str(self.value)
    }
  }

  #[track_caller]
  fn case(escape: bool, indent: &'static str, value: &'static str, expected: &str) {
    let actual = Wrapper {
      escape,
      indent,
      value,
    }
    .to_string();
    assert_eq!(actual, expected);
  }

  #[test]
  fn passthrough() {
    case(false, "", "hello", "hello");
    case(false, "", "", "");
    case(false, "", "\n", "\n");
    case(false, "", "&<>\"'", "&<>\"'");
  }

  #[test]
  fn escape_only() {
    case(true, "", "hello", "hello");
    case(true, "", "\"", "&quot;");
    case(true, "", "&", "&amp;");
    case(true, "", "'", "&apos;");
    case(true, "", "<", "&lt;");
    case(true, "", ">", "&gt;");
    case(true, "", "foo&bar&baz", "foo&amp;bar&amp;baz");
  }

  #[test]
  fn indent_only() {
    case(false, "  ", "a\nb", "a\n  b");
    case(false, "  ", "a\nb\nc", "a\n  b\n  c");
  }

  #[test]
  fn trailing_newline_no_trailing_indent() {
    case(false, "  ", "a\n", "a\n");
    case(false, "  ", "a\nb\n", "a\n  b\n");
  }

  #[test]
  fn empty_value() {
    case(false, "  ", "", "");
    case(true, "  ", "", "");
  }

  #[test]
  fn single_newline() {
    case(false, "  ", "\n", "\n");
    case(true, "  ", "\n", "\n");
  }

  #[test]
  fn only_newlines() {
    case(false, "  ", "\n\n\n", "\n\n\n");
  }

  #[test]
  fn escape_and_indent() {
    case(true, "  ", "a&b\n<c>", "a&amp;b\n  &lt;c&gt;");
  }

  #[test]
  fn escape_immediately_after_newline() {
    case(true, "  ", "\n&", "\n  &amp;");
  }

  #[test]
  fn tab_indent() {
    case(false, "\t", "a\nb", "a\n\tb");
  }

  #[test]
  fn split_writes() {
    struct Sink;

    impl Display for Sink {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = Formatter::new(f, false, "  ");
        f.write_str("a\n")?;
        f.write_str("b")?;
        f.write_str("\nc\n")?;
        Ok(())
      }
    }

    assert_eq!(Sink.to_string(), "a\n  b\n  c\n");
  }

  #[test]
  fn split_at_newline_boundary() {
    struct Sink;

    impl Display for Sink {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = Formatter::new(f, false, "  ");
        f.write_str("a\n")?;
        f.write_str("b")?;
        Ok(())
      }
    }

    assert_eq!(Sink.to_string(), "a\n  b");
  }
}
