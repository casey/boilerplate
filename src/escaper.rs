use super::*;

pub(crate) struct Escaper<'a, 'b>(pub(crate) &'a mut fmt::Formatter<'b>);

impl Write for Escaper<'_, '_> {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    let mut i = 0;

    for (j, c) in s.char_indices() {
      let replacement = match c {
        '"' => "&quot;",
        '&' => "&amp;",
        '<' => "&lt;",
        '>' => "&gt;",
        '\'' => "&apos;",
        _ => continue,
      };

      self.0.write_str(&s[i..j])?;
      self.0.write_str(replacement)?;
      i = j + c.len_utf8();
    }

    self.0.write_str(&s[i..])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(not(feature = "reload"))]
  use alloc::string::ToString;

  struct Wrapper(&'static str);

  impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(Escaper(f), "{}", self.0)
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
