use {
  self::{
    boilerplate::Boilerplate, implementation::Implementation, source::Source, template::Template,
  },
  boilerplate_parser::Token,
  darling::FromDeriveInput,
  new_mime_guess::Mime,
  proc_macro2::{Span, TokenStream},
  quote::{quote, ToTokens, TokenStreamExt},
  std::path::Path,
  syn::{parse_macro_input, DeriveInput, Generics, Ident, LitStr},
};

mod boilerplate;
mod implementation;
mod source;
mod template;

#[proc_macro]
pub fn boilerplate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let template = parse_macro_input!(input as LitStr);
  let src = template.value();

  let Implementation { body, text } = Implementation::parse(&src, false, true);

  quote! {
    {
      extern crate alloc;

      use ::core::fmt::Write;

      let boilerplate_text = &[ #(#text),* ];
      let mut boilerplate_output = alloc::string::String::new();

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
