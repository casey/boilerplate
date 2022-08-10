#[derive(Copy, Clone)]
pub(super) enum Block {
  Code,
  CodeLine,
  Interpolation,
  InterpolationLine,
}

impl Block {
  pub(super) fn starting_at(rest: &str) -> Option<(usize, String)> {
    Self::from_rest(rest).map(|delimiter| delimiter.implementation(rest))
  }

  fn from_rest(rest: &str) -> Option<Self> {
    for kind in [
      Self::Code,
      Self::CodeLine,
      Self::Interpolation,
      Self::InterpolationLine,
    ] {
      if rest.starts_with(kind.open_delimiter()) {
        return Some(kind);
      }
    }

    None
  }

  fn implementation(self, rest: &str) -> (usize, String) {
    let before_open = 0;
    let after_open = before_open + self.open_delimiter().len();
    let before_close = match rest.find(self.close_delimiter()) {
      Some(before_close) => before_close,
      None => panic!("Unmatched `{}`", self.open_delimiter()),
    };
    let after_close = before_close + self.close_delimiter().len();
    let content = &rest[after_open..before_close];

    let rust = match self {
      Self::Code | Self::CodeLine => format!("    {}", content.trim()),
      Self::Interpolation => {
        format!(
          "    f.write_fmt(format_args!(\"{{}}\", {{ {} }}))?;",
          content.trim()
        )
      }
      Self::InterpolationLine => {
        format!(
          "    f.write_fmt(format_args!(\"{{}}\\n\", {{ {} }}))?;",
          content.trim()
        )
      }
    };

    (after_close, rust)
  }

  fn open_delimiter(self) -> &'static str {
    match self {
      Self::Code => "{%",
      Self::CodeLine => "%%",
      Self::Interpolation => "{{",
      Self::InterpolationLine => "$$",
    }
  }

  fn close_delimiter(self) -> &'static str {
    match self {
      Self::Code => "%}",
      Self::CodeLine => "\n",
      Self::Interpolation => "}}",
      Self::InterpolationLine => "\n",
    }
  }
}
