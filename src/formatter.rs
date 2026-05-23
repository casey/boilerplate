use super::*;

/// A `core::fmt::Write` adapter that optionally HTML-escapes its input,
/// optionally appends `indent` after every internal `\n`, and optionally
/// buffers a trailing `\n` so that it can be silently dropped on `Drop`.
#[allow(clippy::struct_excessive_bools)]
pub struct Formatter<'a, W: ?Sized> {
  escape: bool,
  indent: &'static str,
  inner: &'a mut W,
  pending_indent: bool,
  pending_newline: bool,
  trim: bool,
}

impl<'a, W: Write + ?Sized> Formatter<'a, W> {
  pub fn new(inner: &'a mut W, escape: bool, indent: &'static str, trim: bool) -> Self {
    Self {
      escape,
      indent,
      inner,
      pending_indent: false,
      pending_newline: false,
      trim,
    }
  }

  fn flush_newline(&mut self) -> fmt::Result {
    if self.pending_newline {
      self.inner.write_str("\n")?;
      self.pending_indent = true;
      self.pending_newline = false;
    }
    Ok(())
  }

  fn flush_indent(&mut self) -> fmt::Result {
    if self.pending_indent {
      self.inner.write_str(self.indent)?;
      self.pending_indent = false;
    }
    Ok(())
  }

  fn flush(&mut self) -> fmt::Result {
    self.flush_newline()?;
    self.flush_indent()?;
    Ok(())
  }
}

impl<W: Write + ?Sized> Write for Formatter<'_, W> {
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
        self.flush()?;
        self.inner.write_str(&s[chunk_start..i])?;
      }

      if let Some(replacement) = replacement {
        self.flush()?;
        self.inner.write_str(replacement)?;
      } else if self.trim {
        self.flush_newline()?;
        self.pending_newline = true;
      } else {
        self.inner.write_str("\n")?;
        self.pending_indent = true;
      }

      chunk_start = i + 1;
    }

    if chunk_start < s.len() {
      self.flush()?;
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
    trim: bool,
    value: &'static str,
  }

  impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let mut f = Formatter::new(f, self.escape, self.indent, self.trim);
      f.write_str(self.value)
    }
  }

  #[track_caller]
  fn case(escape: bool, indent: &'static str, value: &'static str, expected: &str) {
    let actual = Wrapper {
      escape,
      indent,
      trim: false,
      value,
    }
    .to_string();
    assert_eq!(actual, expected);
  }

  #[track_caller]
  fn trim_case(escape: bool, indent: &'static str, value: &'static str, expected: &str) {
    let actual = Wrapper {
      escape,
      indent,
      trim: true,
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
        let mut f = Formatter::new(f, false, "  ", false);
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
        let mut f = Formatter::new(f, false, "  ", false);
        f.write_str("a\n")?;
        f.write_str("b")?;
        Ok(())
      }
    }

    assert_eq!(Sink.to_string(), "a\n  b");
  }

  #[test]
  fn trim_drops_trailing_newline() {
    trim_case(false, "", "a\n", "a");
    trim_case(false, "", "a", "a");
    trim_case(false, "", "", "");
    trim_case(false, "", "a\nb\n", "a\nb");
    trim_case(false, "", "a\nb", "a\nb");
  }

  #[test]
  fn trim_drops_only_final_newline() {
    trim_case(false, "", "a\n\n", "a\n");
    trim_case(false, "", "\n\n\n", "\n\n");
  }

  #[test]
  fn trim_with_indent() {
    trim_case(false, "  ", "a\nb\n", "a\n  b");
    trim_case(false, "  ", "a\nb", "a\n  b");
    trim_case(false, "  ", "a\n", "a");
  }

  #[test]
  fn trim_with_escape() {
    trim_case(true, "", "&\n", "&amp;");
    trim_case(true, "  ", "a&b\n<c>\n", "a&amp;b\n  &lt;c&gt;");
  }

  #[test]
  fn trim_split_writes() {
    struct Sink;

    impl Display for Sink {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = Formatter::new(f, false, "  ", true);
        f.write_str("a\n")?;
        f.write_str("b\n")?;
        Ok(())
      }
    }

    assert_eq!(Sink.to_string(), "a\n  b");
  }
}
