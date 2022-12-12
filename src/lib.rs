//! `boilerplate` is a template engine.
//!
//! Templates are format agnostic, and can be used to generate HTML, Markdown,
//! or any other text format.
//!
//! The template syntax is very simple, and interpolations and control flow are
//! Rust code, so you don't have to learn a separate template language.
//!
//! Templates are checked at compile time, so any error that the Rust compiler
//! can catch can't make it into production.
//!
//! Template contexts are simple Rust types.
//!
//! `boilerplate` is very simple, requires no runtime dependencies, and is
//! usable in a `no_std` environment.
//!
//! Quick Start
//! -----------
//!
//! Add `boilerplate` to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! boilerplate = "*"
//! ```
//!
//! Create a template in `templates/quick-start.txt`:
//!
//! ```text
//! Foo is {{self.n}}!
//! ```
//!
//! Define, instantiate, and render the template context:
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! struct QuickStartTxt {
//!   n: u32,
//! }
//! assert_eq!(QuickStartTxt { n: 10 }.to_string(), "Foo is 10!\n");
//! ```
//!
//! Template File Locations
//! -----------------------
//!
//! By default, template file paths are relative to the crate root and derived
//! from the context name using the following steps:
//!
//! ```text
//! 1. QuickStartTxt             # get template context name
//! 2. Quick Start Txt           # split words
//! 3. quick start txt           # convert to lowercase
//! 3. quick start.txt           # replace final space with period
//! 4. quick-start.txt           # replace remaining spaces with dashes
//! 6. templates/quick-start.txt # add templates directory
//! ```
//!
//! Inline Templates
//! ----------------
//!
//! Templates contents be read from a string:
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "Hello, world!")]
//! struct Context {}
//! assert_eq!(Context {}.to_string(), "Hello, world!");
//! ```
//!
//! Guide
//! -----
//!
//! Deriving `boilerplate::Boilerplate` on a type generates an implementation of
//! the `Display` trait, which can be printed or rendered to a string with
//! `.to_string()`.
//!
//! Rust code in templates is inserted into the generated `Display::fmt`,
//! function, which takes `&self` as an argument, so it can refer to the
//! template context instance using `self`.
//!
//! ### Text
//!
//! Text is included in template output verbatim.
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "Hello, world!")]
//! struct Context {}
//! assert_eq!(Context {}.to_string(), "Hello, world!");
//! ```
//!
//! ### Interpolation Blocks
//!
//! Expressions inside `{{…}}` are interpolated into the template output:
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "Hello, {{self.name}}!")]
//! struct Context {
//!   name: &'static str,
//! }
//! assert_eq!(Context { name: "Bob" }.to_string(), "Hello, Bob!");
//! ```
//!
//! ### Interpolation Lines
//!
//! Expressions between `$$` and the next newline are interpolated into the
//! template output:
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "My favorite byte is $$ self.byte\n")]
//! struct Context {
//!   byte: u8,
//! }
//! assert_eq!(Context { byte: 38 }.to_string(), "My favorite byte is 38\n");
//! ```
//!
//! ### Code Blocks
//!
//! Code inside of {%…%} is included in the display function body:
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "Knock, knock!
//! {% if !self.alone { %}
//! Who's there?
//! {% } %}
//! ")]
//! struct Context {
//!   alone: bool,
//! }
//! assert_eq!(Context { alone: true }.to_string(), "Knock, knock!\n\n");
//! assert_eq!(Context { alone: false }.to_string(), "Knock, knock!\n\nWho's there?\n\n");
//! ```
//!
//! ### Code Lines
//!
//! Code between `%%` and the next newline is included in the display function
//! body:
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "Knock, knock!
//! %% if !self.alone {
//! Who's there?
//! %% }
//! ")]
//! struct Context {
//!   alone: bool,
//! }
//! assert_eq!(Context { alone: true }.to_string(), "Knock, knock!\n");
//! assert_eq!(Context { alone: false }.to_string(), "Knock, knock!\nWho's there?\n");
//! ```
//!
//! Code lines are often more legible than code blocks. Additionally, becuase
//! the `\n` at the end of a code line is stripped, the rendered templates may
//! include fewer unwanted newlines.
//!
//! ### Loops
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "{% for i in 0..5 { %}Hi!{% } %}")]
//! struct Context {}
//! assert_eq!(Context {}.to_string(), "Hi!Hi!Hi!Hi!Hi!");
//! ```
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "%% for i in 0..10 {
//! {{ i }}
//! %% }
//! ")]
//! struct Context {}
//! assert_eq!(Context {}.to_string(), "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n");
//! ```
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "%% for i in 0..10 {
//! $$ i
//! %% }
//! ")]
//! struct Context {}
//! assert_eq!(Context {}.to_string(), "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n");
//! ```
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "%% for (i, value) in self.0.iter().enumerate() {
//! Value {{i}} is {{value}}
//! %% }
//! ")]
//! struct Context(&'static [&'static str]);
//!
//! assert_eq!(
//!   Context(&["foo", "bar", "baz"]).to_string(),
//!   "Value 0 is foo\nValue 1 is bar\nValue 2 is baz\n"
//! );
//! ```
//!
//! ### Match Statements
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = r#"%% match self.item {
//! %%   Some("foo") => {
//! Found literal foo
//! %%   }
//! %%   Some(val) => {
//! Found {{ val }}
//! %%   }
//! %%   None => {}
//! %% }
//! "#)]
//! struct Context {
//!   item: Option<&'static str>,
//! }
//! assert_eq!(
//!   Context { item: Some("foo") }.to_string(),
//!   "Found literal foo\n"
//! );
//! assert_eq!(Context { item: Some("bar") }.to_string(), "Found bar\n");
//! assert_eq!(Context { item: None }.to_string(), "");
//! ```
//!
//! ### Multiple Statements in an Interpolation
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "$$ { let x = !false; x }\n")]
//! struct Context {}
//! assert_eq!(Context {}.to_string(), "true\n");
//! ```
//!
//! ### The Empty Template
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "")]
//! struct Context {}
//! assert_eq!(Context {}.to_string(), "");
//! ```
//!
//! ### Escaping
//!
//! If the template file path ends with an `html`, `htm`, or `xml` extension,
//! escaping is enabled. Escaping is performed by calling an `escape` method on
//! interpolation values with the following signature:
//!
//! ```
//! trait Escape {
//!   fn escape(&self, f: &mut core::fmt::Formatter, newline: bool) -> core::fmt::Result;
//! }
//! ```
//!
//! Thus, a suitable `Escape` trait must be in scope. The `html-escaper` crate
//! provides just such an `Escape` trait.
//!
//! ```
//! use html_escaper::Escape;
//!
//! #[derive(boilerplate::Boilerplate)]
//! struct EscapeHtml(&'static str);
//! assert_eq!(EscapeHtml("&").to_string(), "&amp;\n");
//! ```
//!
//! ```
//! use html_escaper::Escape;
//!
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "$$ self.0\n")]
//! struct ContextHtml(&'static str);
//! assert_eq!(ContextHtml("&").to_string(), "&amp;\n");
//! ```
//!
//! The `html-escaper` crate also provides a `Trusted` wrapper that disables
//! escaping for trusted values:
//!
//! ```
//! use html_escaper::{Escape, Trusted};
//!
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "$$ Trusted(self.0)\n")]
//! struct ContextHtml(&'static str);
//! assert_eq!(ContextHtml("&").to_string(), "&\n");
//! ```
//!
//! ### Generics
//!
//! Context types may have lifetimes and generics;
//!
//! ```
//! use std::fmt::Display;
//!
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "$$ self.content\n")]
//! struct Context<'a, T: Display> { content: &'a T }
//! assert_eq!(Context { content: &100 }.to_string(), "100\n");
//! ```
//!
//! ### Axum Integration
//!
//! When the `axum` feature is enabled, templates will be provided with an
//! `axum::response::IntoResponse` implementation. The MIME type is deduced
//! from the template path defaulting to `text/plain`. If the MIME type is
//! `text`, `charset=utf-8` will be added automatically, since all
//! boilerplate templates are UTF-8.
//!
//! ```
//! #[cfg(feature = "axum")]
//! {
//!   use axum::response::IntoResponse;
//!   #[derive(boilerplate::Boilerplate)]
//!   struct GuessHtml {}
//!   assert_eq!(
//!     GuessHtml {}
//!       .into_response()
//!       .headers()
//!       .get("content-type")
//!       .unwrap(),
//!     "text/html; charset=utf-8",
//!   );
//! }
//! ```
//!
//! ```
//! #[cfg(feature = "axum")]
//! {
//!   use axum::response::IntoResponse;
//!   #[derive(boilerplate::Boilerplate)]
//!   struct Guess {}
//!   assert_eq!(
//!     Guess {}
//!       .into_response()
//!       .headers()
//!       .get("content-type")
//!       .unwrap(),
//!     "text/plain; charset=utf-8",
//!   );
//! }
//! ```

use {
  self::{block::Block, boilerplate::Boilerplate, source::Source, template::Template},
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

#[proc_macro_derive(Boilerplate, attributes(boilerplate))]
pub fn boilerplate(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = parse_macro_input!(item as DeriveInput);

  Boilerplate::from_derive_input(&derive_input)
    .expect("Failed to parse derive input")
    .impls()
    .into()
}
