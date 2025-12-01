use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Block {
  Code,
  CodeLine,
  Interpolation,
  InterpolationLine,
}

impl Block {
  pub(crate) fn close_delimiter(self) -> &'static str {
    match self {
      Self::Code => "%}",
      Self::CodeLine => "\n",
      Self::Interpolation => "}}",
      Self::InterpolationLine => "\n",
    }
  }

  pub(crate) fn from_rest(rest: &str) -> Option<Self> {
    [
      Self::Code,
      Self::CodeLine,
      Self::Interpolation,
      Self::InterpolationLine,
    ]
    .into_iter()
    .find(|block| rest.starts_with(block.open_delimiter()))
  }

  pub(crate) fn is_line(self) -> bool {
    match self {
      Self::Code | Self::Interpolation => false,
      Self::CodeLine | Self::InterpolationLine => true,
    }
  }

  pub(crate) fn open_delimiter(self) -> &'static str {
    match self {
      Self::Code => "{%",
      Self::CodeLine => "%%",
      Self::Interpolation => "{{",
      Self::InterpolationLine => "$$",
    }
  }

  pub(crate) fn token<'src>(self, contents: &'src str, newline: bool) -> Token<'src> {
    match self {
      Self::Code => Token::Code { contents },
      Self::CodeLine => Token::CodeLine { contents },
      Self::Interpolation => Token::Interpolation { contents },
      Self::InterpolationLine => Token::InterpolationLine { contents, newline },
    }
  }
}
