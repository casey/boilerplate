pub(crate) fn filename_from_ident(ident: &str) -> String {
  let mut words = Vec::new();

  for c in ident.chars() {
    if words.is_empty() || c.is_uppercase() {
      words.push(String::new());
    }

    words.last_mut().unwrap().push(c);
  }

  let mut filename = String::new();

  for (i, word) in words.iter().enumerate() {
    if i > 0 {
      if i == words.len() - 1 {
        filename.push('.');
      } else {
        filename.push('-');
      }
    }
    filename.push_str(word);
  }

  filename.to_lowercase()
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

  #[test]
  fn all_lowercase() {
    assert_eq!(filename_from_ident("foo"), "foo");
  }

  #[test]
  fn camel_case() {
    assert_eq!(filename_from_ident("fooHtml"), "foo.html");
  }
}
