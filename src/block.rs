#[derive(Copy, Clone)]
pub(super) enum Block {
  Code,
  CodeLine,
  Interpolation,
  InterpolationLine,
}

impl Block {
  pub(super) fn parse(rest: &str) -> Option<(usize, String)> {
    Self::from_rest(rest).map(|delimiter| delimiter.output(rest))
  }

  fn from_rest(rest: &str) -> Option<Self> {
    for kind in [
      Self::Code,
      Self::CodeLine,
      Self::Interpolation,
      Self::InterpolationLine,
    ] {
      if rest.starts_with(kind.open()) {
        return Some(kind);
      }
    }

    None
  }

  fn output(self, rest: &str) -> (usize, String) {
    let before_open = 0;
    let after_open = before_open + self.open().len();
    let before_close = match rest.find(self.close()) {
      Some(before_close) => before_close,
      None => panic!("Unmatched `{}`", self.open()),
    };
    let after_close = before_close + self.close().len();
    let content = &rest[after_open..before_close];
    (after_close, self.rust(content))
  }

  fn rust(self, content: &str) -> String {
    match self {
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
    }
  }

  fn open(self) -> &'static str {
    match self {
      Self::Code => "{%",
      Self::CodeLine => "%%",
      Self::Interpolation => "{{",
      Self::InterpolationLine => "$$",
    }
  }

  fn close(self) -> &'static str {
    match self {
      Self::Code => "%}",
      Self::CodeLine => "\n",
      Self::Interpolation => "}}",
      Self::InterpolationLine => "\n",
    }
  }
}
