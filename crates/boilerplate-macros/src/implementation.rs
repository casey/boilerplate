use super::*;

pub(crate) struct Implementation<'src> {
  pub(crate) body: TokenStream,
  pub(crate) text: Vec<&'src str>,
}

impl<'src> Implementation<'src> {
  fn line(token: Token, escape: bool, function: bool) -> String {
    let error_handler = if function { ".unwrap()" } else { "?" };
    match token {
      Token::Text { index, .. } => {
        format!(
          " ::core::fmt::Write::write_str(boilerplate_output, boilerplate_text[{index}].as_ref()){error_handler} ;"
        )
      }
      Token::Code { contents } | Token::CodeLine { contents, .. } => contents.into(),
      Token::Interpolation { contents } => {
        Self::interpolation(false, contents, error_handler, escape)
      }
      Token::InterpolationLine { contents, closed } => {
        Self::interpolation(closed, contents, error_handler, escape)
      }
    }
  }

  fn interpolation(
    append_newline: bool,
    contents: &str,
    error_handler: &str,
    escape: bool,
  ) -> String {
    use std::fmt::Write;

    let mut output = String::new();

    if escape {
      write!(
        output,
        " ::boilerplate::Escape::escape(&({contents}), boilerplate_output)"
      )
      .unwrap();
    } else {
      write!(
        output,
        " ::core::fmt::Write::write_fmt(boilerplate_output, ::core::format_args!(\"{{}}\", {contents}))"
      )
      .unwrap();
    }

    write!(output, "{error_handler} ;").unwrap();

    if append_newline {
      write!(
        output,
        " ::core::fmt::Write::write_str(boilerplate_output, \"\\n\"){error_handler} ;"
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
      .map(|token| Self::line(*token, escape, function))
      .collect::<String>()
      .parse()
      .unwrap();

    Self { body, text }
  }
}
