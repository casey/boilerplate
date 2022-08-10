pub(crate) fn filename_from_ident(ident: &str) -> String {
  let mut filename = String::new();

  for c in ident.chars() {
    if c.is_uppercase() && !filename.is_empty() {
      filename.push('-');
    }
    filename.push_str(&c.to_lowercase().to_string());
  }

  if let Some(last_dash) = filename.rfind('-') {
    filename = format!("{}.{}", &filename[..last_dash], &filename[last_dash + 1..]);
  }

  filename
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    assert_eq!(filename_from_ident("Foo"), "foo");
  }

  #[test]
  fn with_extension() {
    assert_eq!(filename_from_ident("FooHtml"), "foo.html");
  }

  #[test]
  fn multiple_words() {
    assert_eq!(filename_from_ident("FooBarHtml"), "foo-bar.html");
  }

  #[test]
  fn single_letter_words() {
    assert_eq!(filename_from_ident("ABCHtml"), "a-b-c.html");
  }
}
