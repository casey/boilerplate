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
        let trim = followed_by_newline(i, tokens);
        Self::interpolation(contents, indent, trim, false, escape, error_handler)
      }
      Token::InterpolationLine { contents, closed } => {
        let trim = closed || followed_by_newline(i, tokens);
        Self::interpolation(contents, indent, trim, closed, escape, error_handler)
      }
    }
  }

  fn interpolation(
    contents: &str,
    indent: &str,
    trim: bool,
    append_newline: bool,
    escape: bool,
    error_handler: &str,
  ) -> String {
    let write = if escape {
      format!("({contents}).format(boilerplate_output, \"{indent}\", {trim}){error_handler} ;")
    } else if indent.is_empty() && !trim {
      format!("write!(boilerplate_output, \"{{}}\", {contents}){error_handler} ;")
    } else {
      format!(
        "write!(::boilerplate::Formatter::new(boilerplate_output, false, \"{indent}\", {trim}), \
        \"{{}}\", {contents}){error_handler} ;"
      )
    };
    if append_newline {
      format!("{write} boilerplate_output.write_str(\"\\n\"){error_handler} ;")
    } else {
      write
    }
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

fn followed_by_newline(i: usize, tokens: &[Token]) -> bool {
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
  fn case(src: &str, expected: &[&str]) {
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

  #[test]
  fn no_preceding_text() {
    case("{{ x }}", &[""]);
    case("$$ x", &[""]);
  }

  #[test]
  fn indented_alone_on_first_line() {
    case("    {{ x }}", &["    "]);
    case("    $$ x", &["    "]);
  }

  #[test]
  fn tab_indent() {
    case("\t{{ x }}", &["\t"]);
  }

  #[test]
  fn indented_after_newline() {
    case("foo\n    {{ x }}", &["    "]);
    case("foo\n    $$ x", &["    "]);
  }

  #[test]
  fn no_indent_when_text_on_line() {
    case("prefix {{ x }}", &[""]);
    case("foo\nprefix {{ x }}", &[""]);
  }

  #[test]
  fn indented_after_closed_code_line() {
    case("%% if c {\n    {{ x }}\n%% }\n", &["    "]);
  }

  #[test]
  fn indented_after_closed_interpolation_line() {
    case("$$ y\n    {{ x }}", &["", "    "]);
  }

  #[test]
  fn no_indent_when_preceded_by_code_block() {
    case("    {% if c { %}{{ x }}{% } %}", &[""]);
  }

  #[test]
  fn no_indent_when_preceded_by_interpolation() {
    case("    {{ a }}{{ b }}", &["    ", ""]);
  }

  #[test]
  fn multiple_interpolations() {
    case("<body>\n  {{ a }}\n  {{ b }}\n</body>", &["  ", "  "]);
  }

  #[test]
  fn empty_template() {
    case("", &[]);
  }
}
