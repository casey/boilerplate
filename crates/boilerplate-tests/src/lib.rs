#![no_std]

#[allow(unused)]
#[derive(boilerplate::Boilerplate)]
#[boilerplate(axum = false, text = "Hello, {{ self.name }}!")]
struct Context {
  name: &'static str,
}

#[allow(unused)]
fn foo() {
  boilerplate::boilerplate!("Hello!");
}

#[cfg(test)]
mod tests {
  extern crate alloc;

  use alloc::string::ToString;

  #[test]
  fn inherent_escape_method_does_not_shadow_escaped_interpolation() {
    use core::fmt::{self, Display};

    struct Foo;

    impl Foo {
      #[allow(unused, clippy::unused_self)]
      fn escape(&self, _: &str) {
        unreachable!()
      }
    }

    impl Display for Foo {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "&")
      }
    }

    #[derive(boilerplate::Boilerplate)]
    #[boilerplate(axum = false, text = "{{ self.0 }}")]
    struct ContextHtml(Foo);

    assert_eq!(ContextHtml(Foo).to_string(), "&amp;");
  }
}
