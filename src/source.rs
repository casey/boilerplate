use super::*;

pub(crate) enum Source {
  Path(String),
  Literal(LitStr),
}

impl Source {
  pub(crate) fn mime(&self) -> Mime {
    match self {
      Self::Literal(_) => mime::TEXT_PLAIN,
      Self::Path(path) => new_mime_guess::from_path(path).first_or_text_plain(),
    }
  }

  pub(crate) fn text(&self) -> String {
    match self {
      Self::Literal(literal) => literal.value(),
      Self::Path(path) => std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Failed to read template `{path}`: {err}")),
    }
  }
}

impl ToTokens for Source {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match self {
      Self::Literal(literal) => tokens.append(literal.token()),
      Self::Path(path) => {
        let path = LitStr::new(path, Span::call_site());
        tokens.append_all(quote!(include_str!(#path)));
      }
    }
  }
}
