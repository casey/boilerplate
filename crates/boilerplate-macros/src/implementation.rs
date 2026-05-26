use super::*;

pub(crate) struct Implementation<'src> {
  pub(crate) body: TokenStream,
  pub(crate) text: Vec<&'src str>,
}

impl<'src> Implementation<'src> {
  fn line(i: usize, tokens: &[Token<'src>], token: Token, escape: bool, function: bool) -> String {
    let indent = indent(i, tokens).unwrap_or("");
    let error_handler = if function { ".unwrap()" } else { "?" };
    match token {
      Token::Text { index, .. } => {
        format!("boilerplate_output.write_str(boilerplate_text[{index}].as_ref()){error_handler} ;")
      }
      Token::Code { contents } | Token::CodeLine { contents, .. } => contents.into(),
      Token::Interpolation { contents } => {
        let trim = has_trailing_newline(i, tokens);
        Self::interpolation(false, contents, error_handler, escape, indent, trim)
      }
      Token::InterpolationLine { contents, closed } => {
        let trim = closed || has_trailing_newline(i, tokens);
        Self::interpolation(closed, contents, error_handler, escape, indent, trim)
      }
    }
  }

  fn interpolation(
    append_newline: bool,
    contents: &str,
    error_handler: &str,
    escape: bool,
    indent: &str,
    trim: bool,
  ) -> String {
    use std::fmt::Write;

    let mut output = String::new();

    if escape {
      write!(
        output,
        "({contents}).format(boilerplate_output, \"{indent}\", {trim})"
      )
      .unwrap();
    } else if indent.is_empty() && !trim {
      write!(output, "write!(boilerplate_output, \"{{}}\", {contents})").unwrap();
    } else {
      write!(
        output,
        "write!(::boilerplate::Formatter::new(boilerplate_output, false, \"{indent}\", {trim}), \
        \"{{}}\", {contents})"
      )
      .unwrap();
    }

    write!(output, "{error_handler} ;").unwrap();

    if append_newline {
      write!(
        output,
        " boilerplate_output.write_str(\"\\n\"){error_handler} ;"
      )
      .unwrap();
    }

    output
  }

  pub(crate) fn parse(src: &'src str, escape: bool, function: bool) -> Self {
    let tokens = match Token::parse(src) {
      Ok(tokens) => tokens,
      Err(err) => panic!("{err}"),
    };

    let text = tokens.iter().filter_map(|token| token.text()).collect();

    let body = tokens
      .iter()
      .enumerate()
      .map(|(i, token)| Self::line(i, &tokens, *token, escape, function))
      .collect::<String>()
      .parse()
      .unwrap();

    Self { body, text }
  }
}

fn has_trailing_newline(i: usize, tokens: &[Token]) -> bool {
  matches!(
    tokens.get(i + 1),
    Some(Token::Text { contents, .. }) if contents.starts_with('\n'),
  )
}

fn indent<'src>(i: usize, tokens: &[Token<'src>]) -> Option<&'src str> {
  fn is_blank(s: &str) -> bool {
    s.chars().all(|c| c == ' ' || c == '\t')
  }

  if i == 0 {
    return None;
  }

  let Token::Text { contents, .. } = tokens[i - 1] else {
    return None;
  };

  if let Some(newline) = contents.rfind('\n') {
    let prefix = &contents[newline + 1..];
    return is_blank(prefix).then_some(prefix);
  }

  let at_line_start = i < 2
    || matches!(
      tokens[i - 2],
      Token::CodeLine { closed: true, .. } | Token::InterpolationLine { closed: true, .. }
    );

  if !at_line_start {
    return None;
  }

  is_blank(contents).then_some(contents)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[track_caller]
  fn indentation(src: &str, expected: &[&str]) {
    let tokens = Token::parse(src).unwrap();
    let actual = tokens
      .iter()
      .enumerate()
      .filter_map(|(i, token)| match token {
        Token::Interpolation { .. } | Token::InterpolationLine { .. } => {
          Some(indent(i, &tokens).unwrap_or(""))
        }
        _ => None,
      })
      .collect::<Vec<&str>>();
    assert_eq!(actual, expected);
  }

  #[track_caller]
  fn trailing_newlines(src: &str, expected: &[bool]) {
    let tokens = Token::parse(src).unwrap();
    let actual = tokens
      .iter()
      .enumerate()
      .filter_map(|(i, token)| match token {
        Token::Interpolation { .. } | Token::InterpolationLine { .. } => {
          Some(has_trailing_newline(i, &tokens))
        }
        _ => None,
      })
      .collect::<Vec<bool>>();
    assert_eq!(actual, expected);
  }

  #[test]
  fn trailing_newline() {
    trailing_newlines("{{ x }}", &[false]);
    trailing_newlines("{{ x }}\n", &[true]);
    trailing_newlines("{{ x }}foo", &[false]);
    trailing_newlines("{{ x }}\nfoo", &[true]);
    trailing_newlines("{{ x }}{{ y }}", &[false, false]);
    trailing_newlines("{{ x }}{{ y }}\n", &[false, true]);
    trailing_newlines("$$ x", &[false]);
    trailing_newlines("$$ x\n", &[false]);
    trailing_newlines("foo\n{{ x }}", &[false]);
  }

  #[test]
  fn no_preceding_text() {
    indentation("{{ x }}", &[""]);
    indentation("$$ x", &[""]);
  }

  #[test]
  fn indented_alone_on_first_line() {
    indentation("    {{ x }}", &["    "]);
    indentation("    $$ x", &["    "]);
  }

  #[test]
  fn tab_indent() {
    indentation("\t{{ x }}", &["\t"]);
  }

  #[test]
  fn indented_after_newline() {
    indentation("foo\n    {{ x }}", &["    "]);
    indentation("foo\n    $$ x", &["    "]);
  }

  #[test]
  fn no_indent_when_text_on_line() {
    indentation("prefix {{ x }}", &[""]);
    indentation("foo\nprefix {{ x }}", &[""]);
  }

  #[test]
  fn indented_after_closed_code_line() {
    indentation("%% if c {\n    {{ x }}\n%% }\n", &["    "]);
  }

  #[test]
  fn indented_after_closed_interpolation_line() {
    indentation("$$ y\n    {{ x }}", &["", "    "]);
  }

  #[test]
  fn no_indent_when_preceded_by_code_block() {
    indentation("    {% if c { %}{{ x }}{% } %}", &[""]);
  }

  #[test]
  fn no_indent_when_preceded_by_interpolation() {
    indentation("    {{ a }}{{ b }}", &["    ", ""]);
  }

  #[test]
  fn multiple_interpolations() {
    indentation("<body>\n  {{ a }}\n  {{ b }}\n</body>", &["  ", "  "]);
  }

  #[test]
  fn empty_template() {
    indentation("", &[]);
  }
}
