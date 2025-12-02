use super::*;

pub(crate) struct Body<'src> {
  pub(crate) code: TokenStream,
  pub(crate) text: Vec<&'src str>,
  pub(crate) tokens: Vec<Token<'src>>,
}

impl<'src> Body<'src> {
  fn line(token: Token, escape: bool, function: bool) -> String {
    let error_handler = if function { ".unwrap()" } else { "?" };
    match token {
      Token::Text { index, .. } => {
        format!("boilerplate_output.write_str(boilerplate_text[{index}].as_ref()){error_handler} ;",)
      }
      Token::Code { contents } | Token::CodeLine { contents, .. } => contents.into(),
      Token::Interpolation { contents } => {
        if escape {
          format!("({contents}).escape(boilerplate_output, false){error_handler} ;")
        } else {
          format!("write!(boilerplate_output, \"{{}}\", {contents}){error_handler} ;")
        }
      }
      Token::InterpolationLine { contents, closed } => {
        if escape {
          format!("({contents}).escape(boilerplate_output, {closed}){error_handler} ;")
        } else if closed {
          format!("write!(boilerplate_output, \"{{}}\\n\", {contents}){error_handler} ;")
        } else {
          format!("write!(boilerplate_output, \"{{}}\", {contents}){error_handler} ;")
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

    let code = tokens
      .iter()
      .map(|token| Body::line(*token, escape, function))
      .collect::<String>()
      .parse()
      .unwrap();

    Self { code, text, tokens }
  }
}
