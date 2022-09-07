use super::*;

pub(crate) enum Source {
  Path(String),
  Literal(LitStr),
}

impl Source {
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
