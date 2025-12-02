use super::*;

/// Parsed template token.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token<'src> {
  Code { contents: &'src str },
  CodeLine { closed: bool, contents: &'src str },
  Interpolation { contents: &'src str },
  InterpolationLine { closed: bool, contents: &'src str },
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

      let previous_is_code = matches!(
        tokens.last(),
        Some(Token::Code { .. } | Token::CodeLine { .. })
      );

      let current_is_code = matches! {
        block,
        Block::Code | Block::CodeLine,
      };

      if i != j || tokens.is_empty() || !(previous_is_code && current_is_code) {
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

    if i != j || tokens.is_empty() || !matches!(tokens.last(), Some(Token::Text { .. })) {
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
  pub fn is_compatible_with(self, other: Self) -> bool {
    if self.code() != other.code() {
      return false;
    }

    if self.block() != other.block() {
      for token in [self, other] {
        if !matches!(token, Self::Code { .. } | Self::CodeLine { .. }) {
          return false;
        }
      }
    }

    if let Self::InterpolationLine { closed, .. } = self {
      if let Self::InterpolationLine { closed: other, .. } = other {
        if closed != other {
          return false;
        }
      }
    }

    true
  }

  #[must_use]
  pub fn text(self) -> Option<&'src str> {
    if let Self::Text { contents, .. } = self {
      Some(contents)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq, Token::*};

  #[test]
  fn compatibility() {
    #[track_caller]
    fn case(a: Token, b: Token) {
      assert!(a.is_compatible_with(b));
    }
    case(
      Text {
        contents: "foo",
        index: 0,
      },
      Text {
        contents: "bar",
        index: 1,
      },
    );
    case(Code { contents: "foo" }, Code { contents: "foo" });
    case(Code { contents: " foo" }, Code { contents: "foo" });
    case(Code { contents: "foo " }, Code { contents: "foo" });
    case(
      CodeLine {
        contents: "foo",
        closed: true,
      },
      CodeLine {
        contents: "foo",
        closed: true,
      },
    );
    case(
      CodeLine {
        contents: "foo",
        closed: false,
      },
      CodeLine {
        contents: "foo",
        closed: false,
      },
    );
    case(
      CodeLine {
        contents: "foo",
        closed: true,
      },
      CodeLine {
        contents: "foo",
        closed: false,
      },
    );
    case(
      CodeLine {
        contents: "foo",
        closed: false,
      },
      CodeLine {
        contents: "foo",
        closed: true,
      },
    );
    case(
      Code { contents: "foo" },
      CodeLine {
        contents: "foo",
        closed: true,
      },
    );
    case(
      CodeLine {
        contents: "foo",
        closed: true,
      },
      Code { contents: "foo" },
    );
    case(
      Interpolation { contents: "foo" },
      Interpolation { contents: "foo" },
    );
    case(
      InterpolationLine {
        contents: "foo",
        closed: true,
      },
      InterpolationLine {
        contents: "foo",
        closed: true,
      },
    );
    case(
      InterpolationLine {
        contents: "foo",
        closed: false,
      },
      InterpolationLine {
        contents: "foo",
        closed: false,
      },
    );
  }

  #[test]
  fn incompatibility() {
    #[track_caller]
    fn case(a: Token, b: Token) {
      assert!(!a.is_compatible_with(b));
    }
    case(
      Text {
        contents: "foo",
        index: 0,
      },
      Code { contents: "bar" },
    );
    case(Code { contents: "foo" }, Interpolation { contents: "bar" });
    case(
      Interpolation { contents: "foo" },
      InterpolationLine {
        contents: "bar",
        closed: false,
      },
    );
    case(
      InterpolationLine {
        contents: "foo",
        closed: true,
      },
      InterpolationLine {
        contents: "bar",
        closed: true,
      },
    );
    case(
      InterpolationLine {
        contents: "foo",
        closed: true,
      },
      InterpolationLine {
        contents: "foo",
        closed: false,
      },
    );
  }

  #[track_caller]
  fn assert_parse(expected: &str, expected_tokens: &[Token]) {
    let actual_tokens = Token::parse(expected).unwrap();
    assert_eq!(actual_tokens, expected_tokens);
    let actual = actual_tokens
      .iter()
      .map(ToString::to_string)
      .collect::<String>();
    assert_eq!(actual, expected);
  }

  #[test]
  fn empty() {
    assert_parse(
      "",
      &[Text {
        contents: "",
        index: 0,
      }],
    );
  }

  #[test]
  fn text() {
    assert_parse(
      "foo",
      &[Text {
        contents: "foo",
        index: 0,
      }],
    );
  }

  #[test]
  fn code() {
    assert_parse(
      "{% foo %}",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Code { contents: " foo " },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "{%%}",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Code { contents: "" },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
  }

  #[test]
  fn code_line() {
    assert_parse(
      "%% foo\n",
      &[
        Text {
          contents: "",
          index: 0,
        },
        CodeLine {
          contents: " foo",
          closed: true,
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "%% foo",
      &[
        Text {
          contents: "",
          index: 0,
        },
        CodeLine {
          contents: " foo",
          closed: false,
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "%%\n",
      &[
        Text {
          contents: "",
          index: 0,
        },
        CodeLine {
          contents: "",
          closed: true,
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "%%",
      &[
        Text {
          contents: "",
          index: 0,
        },
        CodeLine {
          contents: "",
          closed: false,
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
  }

  #[test]
  fn interpolation() {
    assert_parse(
      "{{ foo }}",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Interpolation { contents: " foo " },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "{{foo}}",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Interpolation { contents: "foo" },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "{{ }}",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Interpolation { contents: " " },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "{{}}",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Interpolation { contents: "" },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
  }

  #[test]
  fn interpolation_line() {
    assert_parse(
      "$$ foo\n",
      &[
        Text {
          contents: "",
          index: 0,
        },
        InterpolationLine {
          contents: " foo",
          closed: true,
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "$$ foo",
      &[
        Text {
          contents: "",
          index: 0,
        },
        InterpolationLine {
          contents: " foo",
          closed: false,
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "$$\n",
      &[
        Text {
          contents: "",
          index: 0,
        },
        InterpolationLine {
          contents: "",
          closed: true,
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "$$",
      &[
        Text {
          contents: "",
          index: 0,
        },
        InterpolationLine {
          contents: "",
          closed: false,
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
  }

  #[test]
  fn mixed() {
    assert_parse(
      "foo {% bar %} baz",
      &[
        Text {
          contents: "foo ",
          index: 0,
        },
        Code { contents: " bar " },
        Text {
          contents: " baz",
          index: 1,
        },
      ],
    );
    assert_parse(
      "{{ foo }} bar {% baz %} bob",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Interpolation { contents: " foo " },
        Text {
          contents: " bar ",
          index: 1,
        },
        Code { contents: " baz " },
        Text {
          contents: " bob",
          index: 2,
        },
      ],
    );
    assert_parse(
      "foo %% bar\nbaz",
      &[
        Text {
          contents: "foo ",
          index: 0,
        },
        CodeLine {
          contents: " bar",
          closed: true,
        },
        Text {
          contents: "baz",
          index: 1,
        },
      ],
    );
    assert_parse(
      "foo $$ bar\nbaz",
      &[
        Text {
          contents: "foo ",
          index: 0,
        },
        InterpolationLine {
          contents: " bar",
          closed: true,
        },
        Text {
          contents: "baz",
          index: 1,
        },
      ],
    );
    assert_parse(
      "{{ foo }}{{ bar }}{{ baz }}",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Interpolation { contents: " foo " },
        Text {
          contents: "",
          index: 1,
        },
        Interpolation { contents: " bar " },
        Text {
          contents: "",
          index: 2,
        },
        Interpolation { contents: " baz " },
        Text {
          contents: "",
          index: 3,
        },
      ],
    );
    assert_parse(
      "a {{ b }} c {{ d }} e",
      &[
        Text {
          contents: "a ",
          index: 0,
        },
        Interpolation { contents: " b " },
        Text {
          contents: " c ",
          index: 1,
        },
        Interpolation { contents: " d " },
        Text {
          contents: " e",
          index: 2,
        },
      ],
    );
    assert_parse(
      "foo {% bar %} baz {% bob %} bill",
      &[
        Text {
          contents: "foo ",
          index: 0,
        },
        Code { contents: " bar " },
        Text {
          contents: " baz ",
          index: 1,
        },
        Code { contents: " bob " },
        Text {
          contents: " bill",
          index: 2,
        },
      ],
    );
    assert_parse(
      "foo %% bar\nbaz %% bob\nbill",
      &[
        Text {
          contents: "foo ",
          index: 0,
        },
        CodeLine {
          contents: " bar",
          closed: true,
        },
        Text {
          contents: "baz ",
          index: 1,
        },
        CodeLine {
          contents: " bob",
          closed: true,
        },
        Text {
          contents: "bill",
          index: 2,
        },
      ],
    );
    assert_parse(
      "text {{ interp }} more {% code %} text %% line\n$$ value\nend",
      &[
        Text {
          contents: "text ",
          index: 0,
        },
        Interpolation {
          contents: " interp ",
        },
        Text {
          contents: " more ",
          index: 1,
        },
        Code { contents: " code " },
        Text {
          contents: " text ",
          index: 2,
        },
        CodeLine {
          contents: " line",
          closed: true,
        },
        Text {
          contents: "",
          index: 3,
        },
        InterpolationLine {
          contents: " value",
          closed: true,
        },
        Text {
          contents: "end",
          index: 4,
        },
      ],
    );
  }

  #[test]
  fn delimiters() {
    assert_parse(
      "{ } % $ %} }}",
      &[Text {
        contents: "{ } % $ %} }}",
        index: 0,
      }],
    );
    assert_parse(
      "%}",
      &[Text {
        contents: "%}",
        index: 0,
      }],
    );
    assert_parse(
      "}}",
      &[Text {
        contents: "}}",
        index: 0,
      }],
    );
  }

  #[test]
  fn nesting() {
    assert_parse(
      "{{ foo {{ bar }}",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Interpolation {
          contents: " foo {{ bar ",
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "{% foo {% bar %}",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Code {
          contents: " foo {% bar ",
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
  }

  #[test]
  fn unicode() {
    assert_parse(
      "Hello 世界",
      &[Text {
        contents: "Hello 世界",
        index: 0,
      }],
    );
    assert_parse(
      "{{ 日本語 }}",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Interpolation {
          contents: " 日本語 ",
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "{% émoji 🚀 %}",
      &[
        Text {
          contents: "",
          index: 0,
        },
        Code {
          contents: " émoji 🚀 ",
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "%% unicode line 中文\n",
      &[
        Text {
          contents: "",
          index: 0,
        },
        CodeLine {
          contents: " unicode line 中文",
          closed: true,
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
    assert_parse(
      "$$ emoji 🎉\n",
      &[
        Text {
          contents: "",
          index: 0,
        },
        InterpolationLine {
          contents: " emoji 🎉",
          closed: true,
        },
        Text {
          contents: "",
          index: 1,
        },
      ],
    );
  }

  #[test]
  fn whitespace() {
    assert_parse(
      "   foo",
      &[Text {
        contents: "   foo",
        index: 0,
      }],
    );
    assert_parse(
      "foo   ",
      &[Text {
        contents: "foo   ",
        index: 0,
      }],
    );
    assert_parse(
      "  {{  foo  }}  ",
      &[
        Text {
          contents: "  ",
          index: 0,
        },
        Interpolation {
          contents: "  foo  ",
        },
        Text {
          contents: "  ",
          index: 1,
        },
      ],
    );
    assert_parse(
      "\t\tfoo\t\t",
      &[Text {
        contents: "\t\tfoo\t\t",
        index: 0,
      }],
    );
    assert_parse(
      "\n\nfoo\n\n",
      &[Text {
        contents: "\n\nfoo\n\n",
        index: 0,
      }],
    );
  }

  #[test]
  fn complex() {
    assert_parse(
      "Hello {{ name }}!
{% for item in items { %}
Item: {{ item }}
{% } %}
Done.",
      &[
        Text {
          contents: "Hello ",
          index: 0,
        },
        Interpolation { contents: " name " },
        Text {
          contents: "!\n",
          index: 1,
        },
        Code {
          contents: " for item in items { ",
        },
        Text {
          contents: "\nItem: ",
          index: 2,
        },
        Interpolation { contents: " item " },
        Text {
          contents: "\n",
          index: 3,
        },
        Code { contents: " } " },
        Text {
          contents: "\nDone.",
          index: 4,
        },
      ],
    );
  }

  #[test]
  fn unclosed() {
    assert_eq!(Token::parse("{%"), Err(Error::Unclosed(Block::Code)),);
    assert_eq!(
      Token::parse("{{"),
      Err(Error::Unclosed(Block::Interpolation)),
    );
  }
}
