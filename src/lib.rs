use {
  self::{
    block::Block, display::Display, filename_from_ident::filename_from_ident, source::Source,
    template::Template,
  },
  darling::FromDeriveInput,
  new_mime_guess::Mime,
  proc_macro2::{Span, TokenStream},
  quote::{quote, ToTokens, TokenStreamExt},
  std::path::Path,
  syn::{DeriveInput, Ident, LitStr},
};

mod block;
mod display;
mod filename_from_ident;
mod source;
mod template;

#[proc_macro_derive(Display, attributes(display))]
pub fn display(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = syn::parse2::<DeriveInput>(TokenStream::from(item))
    .expect("Failed to parse token stream into derive input");

  Display::from_derive_input(&derive_input)
    .expect("Failed to parse derive input")
    .foo()
    .into()
}
