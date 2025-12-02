use super::*;

pub(crate) enum Source {
  Path(String),
  Literal(LitStr),
}

impl Source {
  pub(crate) fn src(&self) -> String {
    match self {
      Self::Literal(literal) => literal.value(),
      Self::Path(path) => std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Failed to read template `{path}`: {err}")),
    }
  }
}
