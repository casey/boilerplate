use super::*;

#[derive(Clone, Copy)]
pub enum Token<'src> {
  Code {
    contents: &'src str,
  },
  CodeLine {
    contents: &'src str,
  },
  Interpolation {
    contents: &'src str,
  },
  InterpolationLine {
    contents: &'src str,
    newline: bool,
  },
  Text {
    start: usize,
    end: usize,
    index: usize,
  },
}

impl<'src> Token<'src> {
  pub fn parse(src: &'src str) -> Vec<Self> {
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

      let (before_close, newline) = match src[after_open..].find(block.close_delimiter()) {
        Some(before_close) => (after_open + before_close, true),
        None if block.is_line() => (src.len(), false),
        None => panic!("unmatched `{}`", block.open_delimiter()),
      };

      let after_close = if newline {
        before_close + block.close_delimiter().len()
      } else {
        before_close
      };

      if i != j {
        tokens.push(Self::Text {
          start: i,
          end: j,
          index,
        });
        index += 1;
      }

      tokens.push(block.token(&src[after_open..before_close], newline));

      j = after_close;
      i = after_close;
    }

    if i != j {
      tokens.push(Self::Text {
        start: i,
        end: j,
        index,
      });
    }

    tokens
  }
}
