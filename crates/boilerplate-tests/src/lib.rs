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
