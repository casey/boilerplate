use super::*;

/// The boilerplate trait, automatically implemented by the `Boilerplate`
/// derive macro.
pub trait Boilerplate {
  /// The original template.
  const TEMPLATE: &'static str;

  /// The parsed template's text blocks.
  const TEXT: &'static [&'static str];

  #[cfg(feature = "reload")]
  /// Path to the original template file.
  const PATH: Option<&'static str>;

  /// Render the template.
  ///
  /// - `boilerplate_text` - The template's text blocks.
  /// - `boilerplate_output` - The formatter to write to.
  fn boilerplate(
    &self,
    boilerplate_text: &[impl AsRef<str>],
    boilerplate_output: &mut fmt::Formatter,
  ) -> fmt::Result;

  #[cfg(feature = "reload")]
  /// Reload the template from a new template string.
  ///
  /// The new template must be compatible with the original template. Templates
  /// are compatible if all of their code blocks, i.e., blocks that contain
  /// Rust code, like `{{ ... }}` are the same. Text blocks, i.e., blocks that
  /// contain literal text, may be different.
  ///
  /// - `src` - The new template source text.
  fn reload(&self, src: &str) -> Result<Reload<&Self>, Error> {
    let new = Token::parse(src).map_err(Error::ParseNew)?;
    let old = Token::parse(Self::TEMPLATE).map_err(Error::ParseOld)?;

    if new.len() != old.len() {
      return Err(Error::Length {
        new: new.len(),
        old: old.len(),
      });
    }

    for (new, old) in new.iter().zip(old) {
      if !new.is_compatible_with(old) {
        return Err(Error::Incompatible {
          new: new.to_string(),
          old: old.to_string(),
        });
      }
    }

    Ok(Reload {
      inner: self,
      text: new
        .into_iter()
        .filter_map(Token::text)
        .map(ToOwned::to_owned)
        .collect(),
    })
  }

  #[cfg(feature = "reload")]
  /// Reload the template from its original path. Cannot be used on templates
  /// created from string literals.
  fn reload_from_path(&self) -> Result<Reload<&Self>, Error> {
    let Some(path) = Self::PATH else {
      return Err(Error::Path);
    };

    let src = std::fs::read_to_string(path).map_err(|source| Error::Io { source, path })?;

    self.reload(&src)
  }
}
