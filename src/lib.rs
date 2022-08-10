use {
  self::{block::Block, filename_from_ident::filename_from_ident},
  darling::FromDeriveInput,
  proc_macro2::TokenStream,
  syn::Ident,
};

mod block;
mod filename_from_ident;

#[derive(FromDeriveInput)]
struct Display {
  ident: Ident,
}

#[proc_macro_derive(Display)]
pub fn display(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input =
    syn::parse2(TokenStream::from(item)).expect("Failed to parse token stream into derive input");

  let Display { ident } =
    Display::from_derive_input(&derive_input).expect("Failed to parse derive input");

  let path = format!(
    "{}/templates/{}",
    std::env::var("CARGO_MANIFEST_DIR")
      .expect("Failed to get `CARGO_MANIFEST_DIR` environment variable"),
    filename_from_ident(&ident.to_string())
  );

  let template = std::fs::read_to_string(&path)
    .unwrap_or_else(|err| panic!("Failed to read template `{path}`: {err}"));

  proc_macro::TokenStream::from(
    impls(&ident, &path, &template)
      .parse::<TokenStream>()
      .expect("Failed to parse display impl"),
  )
}

fn impls(ident: &Ident, path: &str, template: &str) -> String {
  let mut lines = vec![
    format!("impl core::fmt::Display for {ident} {{"),
    "  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {".to_string(),
    format!("    let template = include_str!(\"{path}\");"),
  ];

  let mut i = 0;
  let mut j = 0;
  loop {
    let rest = &template[j..];

    let block = Block::parse(rest);

    if i != j && (block.is_some() || j == template.len()) {
      lines.push(format!("    f.write_str(&template[{}..{}])?;", i, j));
    }

    if j == template.len() {
      break;
    }

    match block {
      Some((end, line)) => {
        lines.push(line);
        j += end;
        i = j;
      }
      None => j += rest.chars().next().unwrap().len_utf8(),
    }
  }

  lines.push("    Ok(())".to_string());
  lines.push("  }".to_string());
  lines.push("}".to_string());

  #[cfg(feature = "axum")]
  {
    lines.push("".to_string());
    lines.push(format!("impl axum::response::IntoResponse for {ident} {{"));
    lines.push(
      "  fn into_response(self) -> axum::response::Response<axum::body::BoxBody> {".to_string(),
    );
    lines.push("    axum::response::Response::builder()".to_string());
    lines.push(format!(
      "      .header(axum::http::header::CONTENT_TYPE, \"{}\")",
      new_mime_guess::from_path(path).first_or_text_plain()
    ));
    lines.push("      .body(axum::body::Full::from(self.to_string()))".to_string());
    lines.push("      .unwrap()".to_string());
    lines.push("      .into_response()".to_string());
    lines.push("  }".to_string());
    lines.push("}".to_string());
  }

  lines.join("\n")
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use {super::*, proc_macro2::Span};

  #[test]
  #[cfg(not(feature = "axum"))]
  fn empty() {
    assert_eq!(
      impls(
        &Ident::new("Foo", Span::call_site()),
        "templates/foo.html",
        ""
      ),
      r#"impl core::fmt::Display for Foo {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let template = include_str!("templates/foo.html");
    Ok(())
  }
}"#
    );
  }

  #[test]
  #[cfg(not(feature = "axum"))]
  fn code() {
    assert_eq!(
      impls(
        &Ident::new("Foo", Span::call_site()),
        "templates/foo.html",
        "{% () %}"
      ),
      r#"impl core::fmt::Display for Foo {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let template = include_str!("templates/foo.html");
    ()
    Ok(())
  }
}"#
    );
  }

  #[test]
  #[cfg(not(feature = "axum"))]
  fn interpolation() {
    assert_eq!(
      impls(
        &Ident::new("Foo", Span::call_site()),
        "templates/foo.html",
        "{{ true }}"
      ),
      r#"impl core::fmt::Display for Foo {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let template = include_str!("templates/foo.html");
    f.write_fmt(format_args!("{}", { true }))?;
    Ok(())
  }
}"#
    );
  }

  #[test]
  #[cfg(not(feature = "axum"))]
  fn text() {
    assert_eq!(
      impls(
        &Ident::new("Foo", Span::call_site()),
        "templates/foo.html",
        "foo"
      ),
      r#"impl core::fmt::Display for Foo {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let template = include_str!("templates/foo.html");
    f.write_str(&template[0..3])?;
    Ok(())
  }
}"#
    );
  }

  #[test]
  #[cfg(feature = "axum")]
  fn axum() {
    assert_eq!(
      impls(
        &Ident::new("Foo", Span::call_site()),
        "templates/foo.html",
        ""
      ),
      r#"impl core::fmt::Display for Foo {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let template = include_str!("templates/foo.html");
    Ok(())
  }
}

impl axum::response::IntoResponse for Foo {
  fn into_response(self) -> axum::response::Response<axum::body::BoxBody> {
    axum::response::Response::builder()
      .header(axum::http::header::CONTENT_TYPE, "text/html")
      .body(axum::body::Full::from(self.to_string()))
      .unwrap()
      .into_response()
  }
}"#
    );
  }
}
