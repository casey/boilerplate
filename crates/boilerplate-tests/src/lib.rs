#![no_std]

#[allow(unused)]
#[derive(boilerplate::Boilerplate)]
#[boilerplate(axum = false, text = "Hello, {{ self.name }}!")]
struct Context {
  name: &'static str,
}

#[allow(unused)]
#[derive(boilerplate::Boilerplate)]
#[boilerplate(
  axum = false,
  text = "<body>
    {{ self.inner }}
</body>
"
)]
struct AutoIndentContext {
  inner: &'static str,
}

#[allow(unused)]
fn foo() {
  boilerplate::boilerplate!("Hello!");
}

#[cfg(test)]
mod tests {
  extern crate alloc;

  use alloc::string::ToString;

  #[derive(boilerplate::Boilerplate)]
  #[boilerplate(
    axum = false,
    text = "<div>
  {{ self.0 }}
</div>
"
  )]
  struct Block(&'static str);

  #[derive(boilerplate::Boilerplate)]
  #[boilerplate(
    axum = false,
    text = "<div>
  $$ self.0
</div>
"
  )]
  struct Line(&'static str);

  #[track_caller]
  fn case(content: &'static str) {
    assert_eq!(Block(content).to_string(), Line(content).to_string());
  }

  #[test]
  fn block_and_line_are_equivalent() {
    case("");
    case("foo");
    case("foo\n");
    case("foo\nbar");
    case("foo\nbar\n");
    case("foo\nbar\nbaz\n");
  }
}
