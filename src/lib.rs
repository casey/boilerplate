//! `boilerplate` is a template engine:
//!
//! Templates are format agnostic, and can be used to generate HTML, Markdown,
//! or any other text format.
//!
//! Template syntax is very simple, and interpolations and control flow are
//! Rust code, so you don't have to learn a separate template language.
//!
//! Templates are checked at compile time, so any error that the Rust compiler
//! can catch can't make it into production.
//!
//! Template contexts are simple Rust types.
//!
//! `boilerplate` is very simple, with the core implementation clocking in at
//! less than 200 lines of code. It requires no runtime dependencies, and is
//! usable in a `no_std` environment.
//!
//! ## Quick Start
//!
//! Add `boilerplate` to your project's `Cargo.toml`:
//!
//! ```toml
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
//! #[derive(boilerplate::Display)]
//! struct QuickStartTxt {
//!   n: u32,
//! }
//! assert_eq!(QuickStartTxt { n: 10 }.to_string(), "Foo is 10!\n");
//! ```
//!
//! ## Template File Locations
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
//! ## Inline Templates
//!
//! Templates contents be read from a string:
//!
//! ```
//! #[derive(boilerplate::Display)]
//! #[display(text = "Hello, world!")]
//! struct Inline {}
//! assert_eq!(Inline {}.to_string(), "Hello, world!");
//! ```
//
//  ## Guide
//
//  Deriving `boilerplate::Display` on a type generates an implementation of
//  the`Display` trait, which can be rendered with `.to_string()` or printed
//  using `{}` in a format macro.
//
//  Code in the template is inserted into the generated `Display::fmt`,
//  function, which takes `&self` as an argument, so template code can refer to
//  the template context using `self`.
//
//  For the following examples, please refer to the accompanying template files
//  in the `templates` directory.
//
//  ### Text
//
//  Text is included in template output verbatim.
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct Text {}
//  assert_eq!(Text {}.to_string(), "Hello, world!\n");
//  ```
//
//  ### Interpolation Blocks
//
//  Expressions inside `{{…}}` are interpolated into the template output:
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct InterpolationBlockTxt {
//    name: &'static str,
//  }
//  assert_eq!(InterpolationBlockTxt { name: "Bob" }.to_string(), "Hello, Bob!\n");
//  ```
//
//  ### Interpolation Lines
//
//  Expressions between `%%` and the end of the line are interpolated into the
//  template output:
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct InterpolationLineTxt {
//    byte: u8,
//  }
//  assert_eq!(InterpolationLineTxt { byte: 38 }.to_string(), "My favorite byte is 38\n");
//  ```
//
//  ### Code Blocks
//
//  Code inside of {%…%} is included in the display function:
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct CodeBlockTxt {
//    alone: bool,
//  }
//  assert_eq!(CodeBlockTxt { alone: true }.to_string(), "Knock, knock!\n\n");
//  assert_eq!(CodeBlockTxt { alone: false }.to_string(), "Knock, knock!\n\nWho's there?\n\n");
//  ```
//
//  ### Code Lines
//
//  Code between `%%` and the end of the line is included in the display:
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct CodeLineTxt {
//    alone: bool,
//  }
//  assert_eq!(CodeLineTxt { alone: true }.to_string(), "Knock, knock!\n");
//  assert_eq!(CodeLineTxt { alone: false }.to_string(), "Knock, knock!\nWho's there?\n");
//  ```
//
//  Code lines are often more legible the code blocks. Additionally, becuase
//  the `\n` at the end of a code line is stripped, the rendered templates may
//  include fewer unwanted newlines.
//
//  ### Loops
//
//  Looping can be performed using code blocks:
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct LoopTxt {}
//  assert_eq!(LoopTxt {}.to_string(), "Hi!\nHi!\nHi!\nHi!\n");
//  ```
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct CodeLineHtml {}
//  assert_eq!(
//    CodeLineHtml {}.to_string(),
//    "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n"
//  );
//  ```
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct EmptyHtml {}
//  assert_eq!(EmptyHtml {}.to_string(), "");
//  ```
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct InterpolationHtml {}
//  assert_eq!(InterpolationHtml {}.to_string(), "true bar false\n");
//  ```
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct InterpolationLineHtml {}
//  assert_eq!(InterpolationLineHtml {}.to_string(), "true\nbar\nfalse\n");
//  ```
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct InterpolationLineMultipleStatementsHtml {}
//  assert_eq!(
//    InterpolationLineMultipleStatementsHtml {}.to_string(),
//    "true\n"
//  );
//  ```
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct InterpolationMultipleStatementsHtml {}
//  assert_eq!(InterpolationMultipleStatementsHtml {}.to_string(), "true\n");
//  ```
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct MatchHtml {
//    item: Option<&'static str>,
//  }
//  assert_eq!(
//    MatchHtml { item: Some("foo") }.to_string(),
//    "Found literal foo\n"
//  );
//  assert_eq!(MatchHtml { item: Some("bar") }.to_string(), "Found bar\n");
//  assert_eq!(MatchHtml { item: None }.to_string(), "");
//  ```
//
//  ```
//  #[derive(boilerplate::Display)]
//  struct TrivialHtml {}
//  assert_eq!(TrivialHtml {}.to_string(), "foo\n");
//  ```

use {
  self::{block::Block, display::Display, source::Source, template::Template},
  darling::FromDeriveInput,
  new_mime_guess::Mime,
  proc_macro2::{Span, TokenStream},
  quote::{quote, ToTokens, TokenStreamExt},
  std::path::Path,
  syn::{DeriveInput, Ident, LitStr},
};

mod block;
mod display;
mod source;
mod template;

#[proc_macro_derive(Display, attributes(display))]
pub fn display(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = syn::parse2::<DeriveInput>(TokenStream::from(item))
    .expect("Failed to parse token stream into derive input");

  Display::from_derive_input(&derive_input)
    .expect("Failed to parse derive input")
    .impls()
    .into()
}
