use super::*;

pub(crate) struct Implementation<'src> {
  pub(crate) body: TokenStream,
  pub(crate) text: Vec<&'src str>,
}

impl<'src> Implementation<'src> {
  fn line(i: usize, tokens: &[Token<'src>], token: Token, escape: bool, function: bool) -> String {
    let indent = detect_indent(i, tokens);
    let error_handler = if function { ".unwrap()" } else { "?" };
    match token {
      Token::Text { index, .. } => {
        format!("boilerplate_output.write_str(boilerplate_text[{index}].as_ref()){error_handler} ;")
      }
      Token::Code { contents } | Token::CodeLine { contents, .. } => contents.into(),
      Token::Interpolation { contents } => {
        if escape {
          format!("({contents}).format(boilerplate_output, \"{indent}\", false){error_handler} ;")
        } else if indent.is_empty() {
          format!("write!(boilerplate_output, \"{{}}\", {contents}){error_handler} ;")
        } else {
          format!(
            "write!(::boilerplate::Formatter::new(boilerplate_output, false, \"{indent}\"), \
            \"{{}}\", {contents}){error_handler} ;"
          )
        }
      }
      Token::InterpolationLine { contents, closed } => {
        if escape {
          format!(
            "({contents}).format(boilerplate_output, \"{indent}\", {closed}){error_handler} ;"
          )
        } else if indent.is_empty() {
          if closed {
            format!("write!(boilerplate_output, \"{{}}\\n\", {contents}){error_handler} ;")
          } else {
            format!("write!(boilerplate_output, \"{{}}\", {contents}){error_handler} ;")
          }
        } else if closed {
          format!(
            "write!(::boilerplate::Formatter::new(boilerplate_output, false, \"{indent}\"), \
            \"{{}}\\n\", {contents}){error_handler} ;"
          )
        } else {
          format!(
            "write!(::boilerplate::Formatter::new(boilerplate_output, false, \"{indent}\"), \
            \"{{}}\", {contents}){error_handler} ;"
          )
        }
      }
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

fn detect_indent<'src>(i: usize, tokens: &[Token<'src>]) -> &'src str {
  if i == 0 {
    return "";
  }

  let Token::Text { contents, .. } = tokens[i - 1] else {
    return "";
  };

  if let Some(newline) = contents.rfind('\n') {
    let prefix = &contents[newline + 1..];
    return if is_indent(prefix) { prefix } else { "" };
  }

  let at_line_start = i < 2
    || matches!(
      tokens[i - 2],
      Token::CodeLine { closed: true, .. } | Token::InterpolationLine { closed: true, .. }
    );

  if at_line_start && is_indent(contents) {
    contents
  } else {
    ""
  }
}

fn is_indent(s: &str) -> bool {
  s.bytes().all(|b| b == b' ' || b == b'\t')
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
          Some(detect_indent(i, &tokens))
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
