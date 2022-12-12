use super::*;

#[derive(FromDeriveInput)]
#[darling(attributes(boilerplate))]
pub(crate) struct Boilerplate {
  generics: Generics,
  ident: Ident,
  text: Option<LitStr>,
}

impl Boilerplate {
  pub(crate) fn impls(self) -> TokenStream {
    let filename = Self::filename_from_ident(&self.ident.to_string());

    let source = match self.text {
      Some(text) => Source::Literal(text),
      None => {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
          .expect("Failed to get `CARGO_MANIFEST_DIR` environment variable");

        let path = Path::new(&manifest_dir).join("templates").join(&filename);

        let path = path.to_str().unwrap_or_else(|| {
          panic!(
            "Path to template `{}` was not valid unicode",
            path.display()
          )
        });

        Source::Path(path.into())
      }
    };

    let escape = Path::new(&filename)
      .extension()
      .map(|extension| ["html", "htm", "xml"].contains(&extension.to_string_lossy().as_ref()))
      .unwrap_or(false);

    let guess = new_mime_guess::from_path(&filename).first_or_text_plain();

    let mime = if guess.type_() == mime::TEXT && guess.get_param(mime::CHARSET).is_none() {
      format!("{}; charset=utf-8", guess).parse().unwrap()
    } else {
      guess
    };

    Template {
      escape,
      generics: self.generics,
      ident: self.ident,
      mime,
      source,
    }
    .impls()
  }

  fn filename_from_ident(ident: &str) -> String {
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
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    assert_eq!(Boilerplate::filename_from_ident("Foo"), "foo");
  }

  #[test]
  fn with_extension() {
    assert_eq!(Boilerplate::filename_from_ident("FooHtml"), "foo.html");
  }

  #[test]
  fn multiple_words() {
    assert_eq!(
      Boilerplate::filename_from_ident("FooBarHtml"),
      "foo-bar.html"
    );
  }

  #[test]
  fn single_letter_words() {
    assert_eq!(Boilerplate::filename_from_ident("ABCHtml"), "a-b-c.html");
  }

  #[test]
  fn all_lowercase() {
    assert_eq!(Boilerplate::filename_from_ident("foo"), "foo");
  }

  #[test]
  fn camel_case() {
    assert_eq!(Boilerplate::filename_from_ident("fooHtml"), "foo.html");
  }
}
