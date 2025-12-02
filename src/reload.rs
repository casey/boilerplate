use {
  super::*,
  std::{fmt::Display, io},
};

/// Reloaded template.
pub struct Reload<T> {
  pub(super) text: Vec<String>,
  pub(super) inner: T,
}

impl<T: Boilerplate> Display for Reload<&T> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    self.inner.boilerplate(&self.text, f)
  }
}

/// Template reload errors.
#[derive(Debug)]
pub enum Error {
  /// New template is not compatible with the old template.
  Incompatible { new: String, old: String },
  /// I/O error loadin new template.
  Io {
    path: &'static str,
    source: io::Error,
  },
  /// New template does not have the same number of blocks.
  Length { new: usize, old: usize },
  /// Failed to parse new template.
  Parse(boilerplate_parser::Error),
  /// Template has no path
  Path,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Incompatible { new, old } => {
        write!(f, "template blocks are not compatible: {new} != {old}")
      }
      Self::Io { path, .. } => write!(f, "I/O error loading template from: {path}"),
      Self::Length { new, old } => write!(
        f,
        "new template has {new} blocks but old template has {old} blocks",
      ),
      Self::Parse(err) => write!(f, "failed to parse new template: {err}"),
      Self::Path => write!(f, "template has no path"),
    }
  }
}

impl core::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    if let Self::Io { source, .. } = self {
      Some(source)
    } else {
      None
    }
  }
}
