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

`boilerplate` is a minimal compile-time Rust text template engine.

## Quick Start

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
struct MyTemplate {
  n: u32,
}

assert_eq!(MyTemplate { n: 10 }.to_string(), "Foo is 10!\n");
```

See [the docs](https://docs.rs/boilerplate/latest/boilerplate/) for more information and examples.
