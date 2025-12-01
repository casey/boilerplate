use super::*;

#[derive(Debug)]
pub enum Error {
  Unclosed(Block),
}

impl std::error::Error for Error {}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Unclosed(block) => write!(f, "unmatched `{}`", block.open_delimiter()),
    }
  }
}
