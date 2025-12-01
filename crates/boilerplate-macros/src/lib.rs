use {
  self::{boilerplate::Boilerplate, source::Source, template::Template},
  boilerplate_parser::Token,
  darling::FromDeriveInput,
  new_mime_guess::Mime,
  proc_macro2::{Span, TokenStream},
  quote::{quote, ToTokens, TokenStreamExt},
  std::path::Path,
  syn::{parse_macro_input, DeriveInput, Generics, Ident, LitStr},
};

mod boilerplate;
mod source;
mod template;

// todo: return value order? maybe a struct?
pub(crate) fn body(
  src: &str,
  escape: bool,
  function: bool,
) -> (TokenStream, Vec<&str>, Vec<Token>) {
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

  let tokens = match Token::parse(src) {
    Ok(tokens) => tokens,
    Err(err) => panic!("{err}"),
  };

  (
    tokens
      .iter()
      .map(|token| line(*token, escape, function))
      .collect::<String>()
      .parse()
      .unwrap(),
    tokens.iter().filter_map(|token| token.text()).collect(),
    tokens,
  )
}

#[proc_macro]
pub fn boilerplate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let template = parse_macro_input!(input as LitStr);
  let text = template.value();

  let (body, template, _tokens) = body(&text, false, true);

  quote! {
    {
      use ::core::fmt::Write;

      let boilerplate_text = &[ #(#template),* ];
      let mut boilerplate_output = String::new();

      #body

      boilerplate_output
    }
  }
  .into()
}

#[allow(non_snake_case)]
#[proc_macro_derive(Boilerplate, attributes(boilerplate))]
pub fn Boilerplate(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = parse_macro_input!(item as DeriveInput);

  Boilerplate::from_derive_input(&derive_input)
    .expect("Failed to parse derive input")
    .impls()
    .into()
}
