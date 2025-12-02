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
//! There are two ways to use boilerplate.
//!
//! One way is with `boilerplate::Boilerplate`, a derive macro, which creates a
//! `Display` implementation on a context type that you provide. The template
//! text is stored in a separate file, and reads variables from the context
//! type when rendered.
//!
//! The other way is with `boilerplate::boilerplate`, a function-like macro,
//! which reads template text from a string literal, and reads variables from
//! the local scope when rendered.
//!
//! Use `boilerplate::Boilerplate` if you want to put your template text in a
//! separate file, or if you need HTML escaping, and `boilerplate::boilerplate`
//! if you want to put your template in a string literal.
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
//! Custom Filename
//! ---------------
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(filename = "quick-start.txt")]
//! struct Context {
//!   n: u32,
//! }
//! assert_eq!(Context { n: 10 }.to_string(), "Foo is 10!\n");
//! ```
//!
//! Inline Templates
//! ----------------
//!
//! Templates contents can be read from a string:
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
//! If there is no following newline, the end of the template text terminates
//! the interpolation:
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "My favorite byte is $$ self.byte")]
//! struct Context {
//!   byte: u8,
//! }
//! assert_eq!(Context { byte: 38 }.to_string(), "My favorite byte is 38");
//! ```
//!
//! This works for escaped templates as well:
//!
//! ```
//! use html_escaper::Escape;
//!
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "My favorite byte is $$ self.byte")]
//! struct ContextHtml {
//!   byte: u8,
//! }
//! assert_eq!(ContextHtml { byte: 38 }.to_string(), "My favorite byte is 38");
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
//! Code lines are often more legible than code blocks. Additionally, because
//! the `\n` at the end of a code line is stripped, the rendered templates may
//! include fewer unwanted newlines.
//!
//! If no newline is present, the code line is terminated by the end of the template
//! text:
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "%% let _x = 2;")]
//! struct Context {}
//! assert_eq!(Context { }.to_string(), "");
//! ```
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
//!
//! ### Reloading Templates
//!
//! When the `reload` feature is enabled, templates support a limited form of
//! hot-reloading.
//!
//! Using `#[derive(Boilerplate]` derives both an implementation of `Display`,
//! and an implementation of the `Boilerplate` trait. Normally the
//! `Boilerplate` trait and its implementation can be ignored, but when the
//! `reload` feature is enabled, the `Boilerplate` trait includes
//! `Boilerplate::reload` which allows a template to be reloaded at runtime.
//!
//! Boilerplate templates contain Rust code which is compiled ahead of time.
//! Consequently, the new template's code blocks must match those of the
//! original template. If they do not, `Boilerplate::reload` will return an
//! error.
//!
//! ```
//! #[cfg(feature = "reload")]
//! {
//!   // import the `Boilerplate` trait for the `reload` method
//!   use boilerplate::Boilerplate;
//!
//!   #[derive(Boilerplate)]
//!   #[boilerplate(text = "Hello, {{self.first}}!")]
//!   struct Context {
//!     first: &'static str,
//!     last: &'static str,
//!   }
//!
//!   let context = Context { first: "Bob", last: "Smith" };
//!   assert_eq!(context.to_string(), "Hello, Bob!");
//!
//!   // Reload a compatible template:
//!   let compatible_template = "Goodbye, {{self.first}}!";
//!   assert_eq!(context.reload(compatible_template).unwrap().to_string(), "Goodbye, Bob!");
//!
//!   // Whitespace around code is allowed to be different:
//!   let compatible_template = "Goodbye, {{ self.first }}!";
//!   assert_eq!(context.reload(compatible_template).unwrap().to_string(), "Goodbye, Bob!");
//!
//!   // Text blocks can be removed entirely:
//!   let incompatible_template = "{{ self.first }}";
//!   assert_eq!(
//!     context.reload(incompatible_template).unwrap().to_string(),
//!     "Bob",
//!   );
//!
//!   // Try to reload an incompatible template with different code:
//!   let incompatible_template = "Goodbye, {{self.id}}!";
//!   assert_eq!(
//!     context.reload(incompatible_template).err().unwrap().to_string(),
//!     "template blocks are not compatible: {{self.id}} != {{self.first}}",
//!   );
//!
//!   // Try to reload an incompatible template with a different number of blocks:
//!   let incompatible_template = "Goodbye, {{self.first}} {{self.last}}!";
//!   assert_eq!(
//!     context.reload(incompatible_template).err().unwrap().to_string(),
//!     "new template has 5 blocks but old template has 3 blocks",
//!   );
//!
//!   // Try to reload a template with invalid syntax:
//!   let incompatible_template = "Goodbye, {{self.first";
//!   assert_eq!(
//!     context.reload(incompatible_template).err().unwrap().to_string(),
//!     "failed to parse new template: unmatched `{{`",
//!   );
//! }
//! ```
//!
//! Mostly, non-code template text can be added, deleted, and removed and still
//! be reload-compatible with the original. The only limitation is that text
//! blocks between `{% ... %}` and `%% ... \n` cannot be inserted or removed.
//!
//! ```
//! #[cfg(feature = "reload")]
//! {
//!   // import the `Boilerplate` trait for the `reload` method
//!   use boilerplate::Boilerplate;
//!
//!   #[derive(Boilerplate)]
//!   #[boilerplate(text = "{% if self.condition { %}{% } %}")]
//!   struct Context {
//!     condition: bool,
//!   }
//!
//!   let context = Context { condition: true };
//!
//!   // Reload a compatible template:
//!   let compatible_template = "{% if self.condition { %}{% } %}";
//!   assert_eq!(context.reload(compatible_template).unwrap().to_string(), "");
//!
//!   // Text between code blocks cannot be inserted:
//!   let incompatible_template = "{% if self.condition { %} hello {% } %}";
//!   assert_eq!(
//!     context.reload(incompatible_template).err().unwrap().to_string(),
//!     "new template has 5 blocks but old template has 4 blocks",
//!   );
//! }
//! ```
//!
//! Text between code blocks can be changed:
//!
//! ```
//! #[cfg(feature = "reload")]
//! {
//!   // import the `Boilerplate` trait for the `reload` method
//!   use boilerplate::Boilerplate;
//!
//!   #[derive(Boilerplate)]
//!   #[boilerplate(text = "{% if self.condition { %}Hello!{% } %}")]
//!   struct Context {
//!     condition: bool,
//!   }
//!
//!   let context = Context { condition: true };
//!   assert_eq!(context.to_string(), "Hello!");
//!
//!   // Reload a compatible template:
//!   let compatible_template = "{% if self.condition { %}Goodbye!{% } %}";
//!   assert_eq!(context.reload(compatible_template).unwrap().to_string(), "Goodbye!");
//! }
//! ```
//!
//! If a template was created from a file, you can call
//! `Boilerplate::reload_from_path` to reload it from its original location:
//!
//! ```
//! #[cfg(feature = "reload")]
//! {
//!   // import the `Boilerplate` trait for the `reload_from_path` method
//!   use boilerplate::Boilerplate;
//!
//!   #[derive(boilerplate::Boilerplate)]
//!   struct QuickStartTxt {
//!     n: u32,
//!   }
//!   assert_eq!(QuickStartTxt { n: 10 }.to_string(), "Foo is 10!\n");
//!   assert_eq!(
//!     QuickStartTxt { n: 10 }.reload_from_path().unwrap().to_string(),
//!     "Foo is 10!\n",
//!   );
//! }
//! ```
//!
//! `Boilerplate::reload_from_path` will return `Error::Path` if the template was
//! created from a string literal:
//!
//! ```
//! #[cfg(feature = "reload")]
//! {
//!   // import the `Boilerplate` trait for the `reload_from_path` method
//!   use boilerplate::Boilerplate;
//!
//!   #[derive(boilerplate::Boilerplate)]
//!   #[boilerplate(text = "Foo is {{ self.n }}!\n")]
//!   struct Context {
//!     n: u32,
//!   }
//!   assert_eq!(Context { n: 10 }.to_string(), "Foo is 10!\n");
//!   assert_eq!(
//!     Context { n: 10 }.reload_from_path().err().unwrap().to_string(),
//!     "template has no path",
//!   );
//! }
//! ```
//!
//! Function-like Macro
//! -------------------
//!
//! A function-like macro named `boilerplate` is also available, which can be
//! used without needing to define a context type.
//!
//! ```
//! use boilerplate::boilerplate;
//!
//! let foo = true;
//! let bar: Result<&str, &str> = Ok("yassss");
//!
//! let output = boilerplate!(
//! "%% if foo {
//! Foo was true!
//! %% }
//! %% match bar {
//! %%   Ok(ok) => {
//! Pretty good: {{ ok }}
//! %%   }
//! %%   Err(err) => {
//! Not so great: {{ err }}
//! %%   }
//! %% }
//! ");
//!
//! assert_eq!(output, "Foo was true!\nPretty good: yassss\n");
//! ```
//!
//! Nesting Templates
//! -----------------
//!
//! Since templates implement `Display` they can used in interpolations inside
//! other templates:
//!
//! ```
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "Hello {{ self.0 }}!")]
//! struct OuterTxt(InnerTxt);
//!
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "Mr. {{ self.0 }}")]
//! struct InnerTxt(&'static str);
//!
//! assert_eq!(OuterTxt(InnerTxt("Miller")).to_string(), "Hello Mr. Miller!");
//! ```
//!
//! This is especially useful when generating multiple HTML pages unique
//! content, but with headers and footers that are common to all pages. Note
//! the use of `html_escaper::Trusted` to prevent escaping the inner HTML:
//!
//! ```
//! use {
//!   html_escaper::{Escape, Trusted},
//!   std::fmt::Display,
//! };
//!
//! trait Page: Display {
//!   fn title(&self) -> &str;
//! }
//!
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "<!doctype html>
//! <html>
//!   <head>
//!     <title>{{ self.0.title() }}</title>
//!   </head>
//!   <body>
//!     {{ Trusted(&self.0) }}
//!   </body>
//! </html>
//! ")]
//! struct OuterHtml<T: Page>(T);
//!
//! #[derive(boilerplate::Boilerplate)]
//! #[boilerplate(text = "<div>{{ self.0 }}</div>")]
//! struct InnerHtml(&'static str);
//!
//! impl Page for InnerHtml {
//!   fn title(&self) -> &str {
//!     "awesome page"
//!   }
//! }
//!
//! assert_eq!(
//!   OuterHtml(InnerHtml("awesome content")).to_string(),
//!   "<!doctype html>
//! <html>
//!   <head>
//!     <title>awesome page</title>
//!   </head>
//!   <body>
//!     <div>awesome content</div>
//!   </body>
//! </html>
//! ");
//! ```

use core::fmt::{self, Formatter};

#[cfg(feature = "reload")]
pub use {
  self::reload::{Error, Reload},
  boilerplate_parser::Token,
};

pub use boilerplate_macros::{boilerplate, Boilerplate};

#[cfg(feature = "reload")]
mod reload;

/// The boilerplate trait, automatically implemented by the `Boilerplate`
/// derive macro.
pub trait Boilerplate {
  /// The parsed template's text blocks.
  const TEXT: &'static [&'static str];

  #[cfg(feature = "reload")]
  /// The parsed template's tokens.
  const TOKENS: &'static [Token<'static>];

  #[cfg(feature = "reload")]
  /// Path to the original template file.
  const PATH: Option<&'static str>;

  /// Render the template.
  ///
  /// - `boilerplate_text` - The template's text blocks.
  /// - `boilerplate_output` - The formatter to write to.
  fn boilerplate(
    &self,
    boilerplate_text: &[impl AsRef<str>],
    boilerplate_output: &mut Formatter,
  ) -> fmt::Result;

  #[cfg(feature = "reload")]
  /// Reload the template from a new template string.
  ///
  /// The new template must be compatible with the original template. Templates
  /// are compatible if all of their code blocks, i.e., blocks that contain
  /// Rust code, like `{{ ... }}` are the same. Text blocks, i.e., blocks that
  /// contain literal text, may be different.
  ///
  /// - `src` - The new template source text.
  fn reload(&self, src: &str) -> Result<Reload<&Self>, Error> {
    let tokens = Token::parse(src).map_err(Error::Parse)?;

    if tokens.len() != Self::TOKENS.len() {
      return Err(Error::Length {
        new: tokens.len(),
        old: Self::TOKENS.len(),
      });
    }

    for (new, old) in tokens.iter().copied().zip(Self::TOKENS.iter().copied()) {
      if !new.is_compatible_with(old) {
        return Err(Error::Incompatible {
          new: new.to_string(),
          old: old.to_string(),
        });
      }
    }

    Ok(Reload {
      inner: self,
      text: tokens
        .into_iter()
        .filter_map(Token::text)
        .map(ToOwned::to_owned)
        .collect(),
    })
  }

  #[cfg(feature = "reload")]
  /// Reload the template from its original path. Cannot be used on templates
  /// created from string literals.
  fn reload_from_path(&self) -> Result<Reload<&Self>, Error> {
    let Some(path) = Self::PATH else {
      return Err(Error::Path);
    };

    let src = std::fs::read_to_string(path).map_err(|source| Error::Io { source, path })?;

    self.reload(&src)
  }
}
