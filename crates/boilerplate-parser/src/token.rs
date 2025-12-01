use super::*;

// todo: add round-trip tests
#[derive(Clone, Copy, Debug)]
pub enum Token<'src> {
  Code { contents: &'src str },
  CodeLine { closed: bool, contents: &'src str },
  Interpolation { contents: &'src str },
  InterpolationLine { contents: &'src str, closed: bool },
  Text { contents: &'src str, index: usize },
}

impl Display for Token<'_> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let block = self.block();

    if let Some(block) = block {
      write!(f, "{}", block.open_delimiter())?;
    }

    write!(f, "{}", self.contents())?;

    match self {
      Self::CodeLine { closed, .. } | Self::InterpolationLine { closed, .. } if !closed => {}
      _ => {
        if let Some(block) = block {
          write!(f, "{}", block.close_delimiter())?;
        }
      }
    }

    Ok(())
  }
}

impl<'src> Token<'src> {
  pub fn parse(src: &'src str) -> Result<Vec<Self>, Error> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut j = 0;
    let mut index = 0;
    while j < src.len() {
      let rest = &src[j..];

      let Some(block) = Block::from_rest(rest) else {
        j += rest.chars().next().unwrap().len_utf8();
        continue;
      };

      let before_open = j;
      let after_open = before_open + block.open_delimiter().len();

      let (before_close, closed) = match src[after_open..].find(block.close_delimiter()) {
        Some(before_close) => (after_open + before_close, true),
        None if block.is_line() => (src.len(), false),
        None => return Err(Error::Unclosed(block)),
      };

      let after_close = if closed {
        before_close + block.close_delimiter().len()
      } else {
        before_close
      };

      if i != j {
        tokens.push(Self::Text {
          contents: &src[i..j],
          index,
        });
        index += 1;
      }

      tokens.push(block.token(&src[after_open..before_close], closed));

      j = after_close;
      i = after_close;
    }

    if i != j {
      tokens.push(Self::Text {
        contents: &src[i..j],
        index,
      });
    }

    Ok(tokens)
  }

  fn code(self) -> Option<&'src str> {
    match self {
      Self::Code { .. }
      | Self::CodeLine { .. }
      | Self::Interpolation { .. }
      | Self::InterpolationLine { .. } => Some(self.contents().trim()),
      Self::Text { .. } => None,
    }
  }

  fn contents(self) -> &'src str {
    match self {
      Self::Code { contents }
      | Self::CodeLine { contents, .. }
      | Self::Interpolation { contents }
      | Self::InterpolationLine { contents, .. }
      | Self::Text { contents, .. } => contents,
    }
  }

  fn block(self) -> Option<Block> {
    match self {
      Self::Code { .. } => Some(Block::Code),
      Self::CodeLine { .. } => Some(Block::CodeLine),
      Self::Interpolation { .. } => Some(Block::Interpolation),
      Self::InterpolationLine { .. } => Some(Block::InterpolationLine),
      Self::Text { .. } => None,
    }
  }

  #[must_use]
  pub fn is_compatible_with(self, other: &Self) -> bool {
    if self.code() != other.code() {
      return false;
    }

    match (self, other) {
      (Self::Code { .. }, Self::Code { .. })
      | (Self::Interpolation { .. }, Self::Interpolation { .. })
      | (Self::Text { .. }, Self::Text { .. }) => true,
      (Self::CodeLine { closed, .. }, Self::CodeLine { closed: other, .. })
      | (Self::InterpolationLine { closed, .. }, Self::InterpolationLine { closed: other, .. }) => {
        closed == *other
      }
      _ => false,
    }
  }

  // todo: can I replate this with call to contents?
  #[must_use]
  pub fn text(self) -> Option<&'src str> {
    if let Self::Text { contents, .. } = self {
      Some(contents)
    } else {
      None
    }
  }
}
