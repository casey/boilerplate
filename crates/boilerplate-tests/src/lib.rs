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
  fn equivalent(content: &'static str) {
    assert_eq!(Block(content).to_string(), Line(content).to_string());
  }

  #[test]
  fn block_and_line_are_equivalent() {
    equivalent("");
    equivalent("foo");
    equivalent("foo\n");
    equivalent("foo\nbar");
    equivalent("foo\nbar\n");
    equivalent("foo\nbar\nbaz\n");
  }
}
