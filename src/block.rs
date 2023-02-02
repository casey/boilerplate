use super::*;

#[derive(Copy, Clone)]
pub(super) enum Block {
  Code,
  CodeLine,
  Interpolation,
  InterpolationLine,
}

impl Block {
  pub(crate) fn body(text: &str, escape: bool, function: bool) -> TokenStream {
    let error_handler = if function { ".unwrap()" } else { "?" };
    let mut lines = Vec::new();
    let mut i = 0;
    let mut j = 0;
    loop {
      let rest = &text[j..];

      let block = Self::implementation_starting_at(rest, escape, error_handler);

      if i < j && block.is_some() {
        lines.push(format!(
          "boilerplate_output.write_str(&boilerplate_template[{i}..{j}]){error_handler} ;",
        ));
      }

      if i < j && j == text.len() {
        lines.push(format!(
          "boilerplate_output.write_str(&boilerplate_template[{i}..]){error_handler} ;",
        ));
      }

      if j == text.len() {
        break;
      }

      match block {
        Some((length, line)) => {
          lines.push(line);
          j += length;
          i = j;
        }
        None => j += rest.chars().next().unwrap().len_utf8(),
      }
    }

    lines.join("").parse().unwrap()
  }

  fn implementation_starting_at(
    rest: &str,
    escape: bool,
    error_handler: &str,
  ) -> Option<(usize, String)> {
    Some(Self::from_rest(rest)?.implementation(rest, escape, error_handler))
  }

  fn from_rest(rest: &str) -> Option<Self> {
    [
      Self::Code,
      Self::CodeLine,
      Self::Interpolation,
      Self::InterpolationLine,
    ]
    .into_iter()
    .find(|variant| rest.starts_with(variant.open_delimiter()))
  }

  fn implementation(self, rest: &str, escape: bool, error_handler: &str) -> (usize, String) {
    let before_open = 0;
    let after_open = before_open + self.open_delimiter().len();
    let (before_close, newline) = match rest.find(self.close_delimiter()) {
      Some(before_close) => (before_close, true),
      None if self.is_line() => (rest.len(), false),
      None => panic!("Unmatched `{}`", self.open_delimiter()),
    };

    let after_close = if newline {
      before_close + self.close_delimiter().len()
    } else {
      before_close
    };

    let contents = &rest[after_open..before_close];

    let rust = match self {
      Self::Code | Self::CodeLine => contents.into(),
      Self::Interpolation => {
        if escape {
          format!("({contents}).escape(boilerplate_output, false){error_handler} ;")
        } else {
          format!("write!(boilerplate_output, \"{{}}\", {contents}){error_handler} ;")
        }
      }
      Self::InterpolationLine => {
        if escape {
          format!("({contents}).escape(boilerplate_output, {newline}){error_handler} ;")
        } else if newline {
          format!("write!(boilerplate_output, \"{{}}\\n\", {contents}){error_handler} ;")
        } else {
          format!("write!(boilerplate_output, \"{{}}\", {contents}){error_handler} ;")
        }
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

  fn is_line(self) -> bool {
    match self {
      Self::Code | Self::Interpolation => false,
      Self::CodeLine | Self::InterpolationLine => true,
    }
  }
}
