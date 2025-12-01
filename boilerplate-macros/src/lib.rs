use {
  self::{
    block::Block, boilerplate::Boilerplate, source::Source, template::Template, token::Token,
  },
  darling::FromDeriveInput,
  new_mime_guess::Mime,
  proc_macro2::{Span, TokenStream},
  quote::{quote, ToTokens, TokenStreamExt},
  std::path::Path,
  syn::{parse_macro_input, DeriveInput, Generics, Ident, LitStr},
};

mod block;
mod boilerplate;
mod source;
mod template;
mod token;

pub(crate) fn body(src: &str, escape: bool, function: bool) -> (TokenStream, Vec<LitStr>) {
  let (tokens, text) = Token::parse(src);

  (
    tokens
      .iter()
      .map(|token| token.line(escape, function))
      .collect::<Vec<String>>()
      .join("")
      .parse()
      .unwrap(),
    text
      .iter()
      .map(|s| LitStr::new(s, Span::call_site()))
      .collect::<Vec<LitStr>>(),
  )
}

#[proc_macro]
pub fn boilerplate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let template = parse_macro_input!(input as LitStr);
  let text = template.value();

  let (body, template) = body(&text, false, true);

  quote! {
    {
      use ::core::fmt::Write;

      let boilerplate_template = &[ #(#template),* ];
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
