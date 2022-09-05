use super::*;

pub(crate) enum Source {
  Path(String),
  Literal(LitStr),
}

impl Source {
  pub(crate) fn mime(&self) -> Mime {
    match self {
      Source::Literal(_) => mime::TEXT_PLAIN,
      Source::Path(path) => new_mime_guess::from_path(path).first_or_text_plain(),
    }
  }
}

impl ToTokens for Source {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match self {
      Source::Literal(literal) => tokens.append(literal.token()),
      Source::Path(path) => {
        let path = LitStr::new(&path, Span::call_site());
        tokens.append_all(quote!(include_str!(#path)));
      }
    }
  }
}
