<h1 align="center"><code>boilerplate</code></h1>

<div align="center">
  <a href="https://crates.io/crates/boilerplate">
    <img src="https://img.shields.io/crates/v/boilerplate.svg" alt="crates.io version">
  </a>
  <a href="https://docs.rs/boilerplate/latest/boilerplate/">
    <img src="https://img.shields.io/crates/v/boilerplate?color=blue&label=docs" alt="docs">
  </a>
  <a href="https://github.com/casey/boilerplate/actions">
    <img src="https://github.com/casey/boilerplate/workflows/CI/badge.svg" alt="ci status">
  </a>
</div>

<br>

`boilerplate` is a statically-checked Rust template engine with no runtime
dependencies. There are two ways to use boilerplate,
`boilerplate::boilerplate`, a function-like macro, and
`boilerplate::Boilerplate`, a derive macro.

Function-like Macro
-------------------

```rust
use boilerplate::boilerplate;

let foo = true;
let bar: Result<&str, &str> = Ok("yassss");

let output = boilerplate!(
"%% if foo {
Foo was true!
%% }
%% match bar {
%%   Ok(ok) => {
Pretty good: {{ ok }}
%%   }
%%   Err(err) => {
Not so great: {{ err }}
%%   }
%% }
");

assert_eq!(output, "Foo was true!\nPretty good: yassss\n");
```

Derive Macro
------------

Derive `Boilerplate` on the type you want to use as a template context:

```rust
use boilerplate::Boilerplate;

#[derive(Boilerplate)]
struct MyTemplateTxt {
  foo: bool,
  bar: Result<String, Box<dyn std::error::Error>>,
}
```

`boilerplate` template code and interpolations are Rust, so errors are checked
at compile time and the template language is easy to learn:

```text
%% if self.foo {
Foo was true!
%% }
%% match &self.bar {
%%   Ok(ok) => {
Pretty good: {{ ok }}
%%   }
%%   Err(err) => {
Not so great: {{ err }}
%%   }
%% }
```

The `Boilerplate` macro provides a `Display` implementation, so you can
instantiate a template context and convert it into a string:

```rust
let rendered = MyTemplateTxt { foo: true, bar: Ok("hello".into()) }.to_string();
```

Or use it in a format string:

```rust
println!("The output is: {}", MyTemplateTxt { foo: false, bar: Err("hello".into()) });
```

`boilerplate`'s implementation is exceedingly simple. Try using
[cargo-expand](https://github.com/dtolnay/cargo-expand) to expand the
`Boilerplate` macro and inspect derived `Display` implementations and debug
template issues.

Quick Start
-----------

Add `boilerplate` to your project's `Cargo.toml`:

```toml
[dependencies]
boilerplate = "*"
```

Create a template in `templates/my-template.txt`:

```text
Foo is {{self.n}}!
```

Define, instantiate, and render the template context:

```rust
use boilerplate::Boilerplate;

#[derive(Boilerplate)]
struct MyTemplateTxt {
  n: u32,
}

assert_eq!(MyTemplateTxt  { n: 10 }.to_string(), "Foo is 10!\n");
```

Examples
--------

See [the docs](https://docs.rs/boilerplate/latest/boilerplate/) for more information and examples.
