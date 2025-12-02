use super::*;

/// Parsed template token.
#[derive(Clone, Copy, Debug)]
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
  use super::*;

  /// Helper function to test round-trip parsing
  fn assert_round_trip(src: &str) {
    let tokens = Token::parse(src).expect("Failed to parse");
    let reconstructed: String = tokens.iter().map(ToString::to_string).collect();
    assert_eq!(
      src, reconstructed,
      "Round-trip failed:\nOriginal:      {src:?}\nReconstructed: {reconstructed:?}",
    );
  }

  #[test]
  fn round_trip_empty_string() {
    assert_round_trip("");
  }

  #[test]
  fn round_trip_plain_text() {
    assert_round_trip("hello world");
    assert_round_trip("simple text");
    assert_round_trip("text with spaces   and   gaps");
  }

  #[test]
  fn round_trip_code_block() {
    assert_round_trip("{% code %}");
    assert_round_trip("{%code%}");
    assert_round_trip("{% code with spaces %}");
    assert_round_trip("{%  %}");
    assert_round_trip("{% %}");
  }

  #[test]
  fn round_trip_code_line() {
    assert_round_trip("%% code line\n");
    assert_round_trip("%%code line\n");
    assert_round_trip("%% code with spaces\n");
    assert_round_trip("%%  \n");
    assert_round_trip("%%\n");
  }

  #[test]
  fn round_trip_code_line_unclosed() {
    assert_round_trip("%% unclosed line");
    assert_round_trip("%%unclosed");
    assert_round_trip("%%  ");
    assert_round_trip("%%");
  }

  #[test]
  fn round_trip_interpolation() {
    assert_round_trip("{{ value }}");
    assert_round_trip("{{value}}");
    assert_round_trip("{{ value with spaces }}");
    assert_round_trip("{{  }}");
    assert_round_trip("{{}}");
  }

  #[test]
  fn round_trip_interpolation_line() {
    assert_round_trip("$$ value\n");
    assert_round_trip("$$value\n");
    assert_round_trip("$$ value with spaces\n");
    assert_round_trip("$$  \n");
    assert_round_trip("$$\n");
  }

  #[test]
  fn round_trip_interpolation_line_unclosed() {
    assert_round_trip("$$ unclosed line");
    assert_round_trip("$$unclosed");
    assert_round_trip("$$  ");
    assert_round_trip("$$");
  }

  #[test]
  fn round_trip_mixed_tokens() {
    assert_round_trip("text {% code %} more text");
    assert_round_trip("{{ x }} and {% y %} text");
    assert_round_trip("start %% line\nend");
    assert_round_trip("start $$ line\nend");
    assert_round_trip("{{ a }}{{ b }}{{ c }}");
  }

  #[test]
  fn round_trip_multiple_text_segments() {
    assert_round_trip("a {{ b }} c {{ d }} e");
    assert_round_trip("text {% code %} middle {% more %} end");
    assert_round_trip("one %% two\nthree %% four\nfive");
  }

  #[test]
  fn round_trip_delimiters_in_text() {
    // Partial delimiters that don't form complete opening sequences
    assert_round_trip("text with } in it");
    assert_round_trip("text with % in it");
    assert_round_trip("text with $ in it");
    assert_round_trip("text with { alone");
    // Close delimiters without opens are fine
    assert_round_trip("text with }} in it");
    assert_round_trip("text with %} in it");
  }

  #[test]
  fn round_trip_nested_delimiters_in_content() {
    assert_round_trip("{{ content with {{ nested }}");
    assert_round_trip("{% content with {% nested %}");
  }

  #[test]
  fn round_trip_unicode() {
    assert_round_trip("Hello 世界");
    assert_round_trip("{{ 日本語 }}");
    assert_round_trip("{% émoji 🚀 %}");
    assert_round_trip("%% unicode line 中文\n");
    assert_round_trip("$$ emoji 🎉\n");
  }

  #[test]
  fn round_trip_special_characters() {
    assert_round_trip("{{ \t\r }}");
    assert_round_trip("{% newline\nhere %}");
    assert_round_trip("text\nwith\nnewlines");
    assert_round_trip("{{ \" quote \" }}");
    assert_round_trip("{% ' apostrophe ' %}");
  }

  #[test]
  fn round_trip_whitespace() {
    assert_round_trip("   leading spaces");
    assert_round_trip("trailing spaces   ");
    assert_round_trip("  {{  spaces  }}  ");
    assert_round_trip("\t\ttabs\t\t");
    assert_round_trip("\n\nnewlines\n\n");
  }

  #[test]
  fn round_trip_consecutive_blocks() {
    assert_round_trip("{{a}}{{b}}{{c}}");
    assert_round_trip("{%a%}{%b%}{%c%}");
    assert_round_trip("%%a\n%%b\n%%c\n");
    assert_round_trip("$$a\n$$b\n$$c\n");
  }

  #[test]
  fn round_trip_empty_blocks() {
    assert_round_trip("{{}}");
    assert_round_trip("{%%}");
    assert_round_trip("%%\n");
    assert_round_trip("$$\n");
  }

  #[test]
  fn round_trip_complex_template() {
    assert_round_trip(
      "Hello {{ name }}!\n\
       {% for item in items %}\n\
       Item: {{ item }}\n\
       {% endfor %}\n\
       Done.",
    );
  }

  #[test]
  fn round_trip_all_token_types_mixed() {
    assert_round_trip("text {{ interp }} more {% code %} text %% line\n$$ value\nend");
  }

  #[test]
  fn round_trip_edge_case_sequences() {
    // Close delimiters without opens
    assert_round_trip("%}}}");
    assert_round_trip("}}%}");
    // Line delimiters at start
    assert_round_trip("\n%% line\n");
    assert_round_trip("\n$$ line\n");
  }

  #[test]
  fn round_trip_long_content() {
    assert_round_trip("{{ this is a very long interpolation with lots of text inside }}");
    assert_round_trip("{% this is a very long code block with lots of text inside %}");
    assert_round_trip("This is a very long text segment that goes on and on without any tokens at all just plain text");
  }

  #[test]
  fn round_trip_mixed_line_endings() {
    assert_round_trip("%% line1\n%% line2\ntext\n%% line3\n");
    assert_round_trip("$$ val1\n$$ val2\ntext\n$$ val3\n");
  }

  #[test]
  fn round_trip_unclosed_at_end() {
    assert_round_trip("text %% unclosed");
    assert_round_trip("text $$ unclosed");
    assert_round_trip("%% only unclosed");
    assert_round_trip("$$ only unclosed");
  }

  #[test]
  fn round_trip_closed_and_unclosed_mixed() {
    assert_round_trip("%% closed\n%% unclosed");
    assert_round_trip("$$ closed\n$$ unclosed");
    assert_round_trip("%% a\n%% b\n%% c");
  }

  #[test]
  fn round_trip_only_delimiters() {
    assert_round_trip("{{}}{{}}");
    assert_round_trip("{%%}{%%}");
    assert_round_trip("%%%%");
    assert_round_trip("$$$$");
  }

  #[test]
  fn round_trip_interleaved_types() {
    assert_round_trip("{{ a }}{% b %}{{ c }}{% d %}");
    assert_round_trip("%% a\n$$ b\n%% c\n$$ d\n");
    assert_round_trip("{{ a }}%% b\n{% c %}$$ d\n");
  }

  #[test]
  fn unclosed_code_block_error() {
    let result = Token::parse("{% unclosed");
    assert!(result.is_err());
    if let Err(Error::Unclosed(block)) = result {
      assert_eq!(block.open_delimiter(), "{%");
    }
  }

  #[test]
  fn unclosed_interpolation_error() {
    let result = Token::parse("{{ unclosed");
    assert!(result.is_err());
    if let Err(Error::Unclosed(block)) = result {
      assert_eq!(block.open_delimiter(), "{{");
    }
  }

  #[test]
  fn unclosed_code_block_with_text_before() {
    let result = Token::parse("text {% unclosed");
    assert!(result.is_err());
    if let Err(Error::Unclosed(block)) = result {
      assert_eq!(block.open_delimiter(), "{%");
    }
  }

  #[test]
  fn unclosed_interpolation_with_text_before() {
    let result = Token::parse("text {{ unclosed");
    assert!(result.is_err());
    if let Err(Error::Unclosed(block)) = result {
      assert_eq!(block.open_delimiter(), "{{");
    }
  }

  #[test]
  fn unclosed_code_block_with_text_after() {
    let result = Token::parse("{% unclosed after");
    assert!(result.is_err());
  }

  #[test]
  fn unclosed_interpolation_with_text_after() {
    let result = Token::parse("{{ unclosed after");
    assert!(result.is_err());
  }

  #[test]
  fn multiple_unclosed_first_fails() {
    // First unclosed block causes error
    let result = Token::parse("{% first {{ second");
    assert!(result.is_err());
  }

  #[test]
  fn parse_empty() {
    let tokens = Token::parse("").unwrap();
    assert_eq!(tokens.len(), 0);
  }

  #[test]
  fn parse_only_text() {
    let tokens = Token::parse("just text").unwrap();
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Text { .. }));
  }

  #[test]
  fn parse_only_code() {
    let tokens = Token::parse("{% code %}").unwrap();
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Code { .. }));
  }

  #[test]
  fn parse_only_interpolation() {
    let tokens = Token::parse("{{ value }}").unwrap();
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Interpolation { .. }));
  }

  #[test]
  fn parse_only_code_line_closed() {
    let tokens = Token::parse("%% line\n").unwrap();
    assert_eq!(tokens.len(), 1);
    if let Token::CodeLine { closed, .. } = tokens[0] {
      assert!(closed);
    } else {
      panic!("Expected CodeLine token");
    }
  }

  #[test]
  fn parse_only_code_line_unclosed() {
    let tokens = Token::parse("%% line").unwrap();
    assert_eq!(tokens.len(), 1);
    if let Token::CodeLine { closed, .. } = tokens[0] {
      assert!(!closed);
    } else {
      panic!("Expected CodeLine token");
    }
  }

  #[test]
  fn parse_only_interpolation_line_closed() {
    let tokens = Token::parse("$$ value\n").unwrap();
    assert_eq!(tokens.len(), 1);
    if let Token::InterpolationLine { closed, .. } = tokens[0] {
      assert!(closed);
    } else {
      panic!("Expected InterpolationLine token");
    }
  }

  #[test]
  fn parse_only_interpolation_line_unclosed() {
    let tokens = Token::parse("$$ value").unwrap();
    assert_eq!(tokens.len(), 1);
    if let Token::InterpolationLine { closed, .. } = tokens[0] {
      assert!(!closed);
    } else {
      panic!("Expected InterpolationLine token");
    }
  }

  #[test]
  fn token_contents() {
    let tokens = Token::parse("{{ value }}").unwrap();
    assert_eq!(tokens[0].contents(), " value ");
  }

  #[test]
  fn token_text() {
    let tokens = Token::parse("text").unwrap();
    assert_eq!(tokens[0].text(), Some("text"));

    let tokens = Token::parse("{{ value }}").unwrap();
    assert_eq!(tokens[0].text(), None);
  }

  #[test]
  fn token_is_compatible_with() {
    let tokens1 = Token::parse("{{ value }}").unwrap();
    let tokens2 = Token::parse("{{ value }}").unwrap();
    assert!(tokens1[0].is_compatible_with(&tokens2[0]));

    let tokens3 = Token::parse("{{ other }}").unwrap();
    assert!(!tokens1[0].is_compatible_with(&tokens3[0]));
  }
}
