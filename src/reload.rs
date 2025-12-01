use {super::*, core::fmt::Display};

pub struct Reload<T> {
  pub(super) text: Vec<String>,
  pub(super) inner: T,
}

impl<T: Boilerplate> Display for Reload<&T> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    self.inner.boilerplate(&self.text, f)
  }
}

#[derive(Debug)]
pub enum Error<'a> {
  Incompatible { new: Token<'a>, old: Token<'a> },
  Length { new: usize, old: usize },
  Parse(boilerplate_parser::Error),
}

impl Display for Error<'_> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Incompatible { new, old } => {
        write!(f, "template blocks are not compatible: {new} != {old}",)
      }
      Self::Length { new, old } => write!(
        f,
        "new and old template block length mismatch: {new} != {old}"
      ),
      Self::Parse(err) => write!(f, "failed to parse new template: {err}"),
    }
  }
}

impl core::error::Error for Error<'_> {}
