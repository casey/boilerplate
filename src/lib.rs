use {
  self::{block::Block, filename_from_ident::filename_from_ident},
  proc_macro2::TokenStream,
  std::path::Path,
  syn::{DeriveInput, Ident},
};

mod block;
mod filename_from_ident;

#[proc_macro_derive(Display)]
pub fn display(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ident = syn::parse2::<DeriveInput>(TokenStream::from(item))
    .expect("Failed to parse token stream into derive input")
    .ident;

  let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
    .expect("Failed to get `CARGO_MANIFEST_DIR` environment variable");

  let path = Path::new(&manifest_dir)
    .join("templates")
    .join(filename_from_ident(&ident.to_string()));

  let template = std::fs::read_to_string(&path)
    .unwrap_or_else(|err| panic!("Failed to read template `{}`: {err}", path.display()));

  let path_unicode = path.to_str().unwrap_or_else(|| {
    panic!(
      "Path to template `{}` was not valid unicode",
      path.display()
    )
  });

  proc_macro::TokenStream::from(
    impls(&ident, path_unicode, &template)
      .parse::<TokenStream>()
      .expect("Failed to parse display impl"),
  )
}

fn impls(ident: &Ident, path: &str, template: &str) -> String {
  let mut lines = vec![
    format!("impl core::fmt::Display for {ident} {{"),
    "  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {".to_string(),
    format!(
      "    let text = include_str!(\"{}\");",
      path.escape_default()
    ),
  ];

  let mut i = 0;
  let mut j = 0;
  loop {
    let rest = &template[j..];

    let block = Block::starting_at(rest);

    if i < j && block.is_some() {
      lines.push(format!("    f.write_str(&text[{}..{}])?;", i, j));
    }

    if i < j && j == template.len() {
      lines.push(format!("    f.write_str(&text[{}..])?;", i));
    }

    if j == template.len() {
      break;
    }

    match block {
      Some((length, line)) => {
        lines.push(line);
        j += length;
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
  use {super::*, pretty_assertions::assert_eq, proc_macro2::Span, unindent::Unindent};

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
             let text = include_str!("templates/foo.html");
             Ok(())
           }
         }"#
        .unindent()
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
            let text = include_str!("templates/foo.html");
            ()
            Ok(())
          }
        }"#
        .unindent()
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
             let text = include_str!("templates/foo.html");
             write!(f, "{}", true)?;
             Ok(())
           }
         }"#
        .unindent()
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
             let text = include_str!("templates/foo.html");
             f.write_str(&text[0..])?;
             Ok(())
           }
         }"#
        .unindent()
    );
  }

  #[test]
  #[cfg(feature = "axum")]
  fn axum_guess_html() {
    assert_eq!(
      impls(
        &Ident::new("Foo", Span::call_site()),
        "templates/foo.html",
        ""
      ),
      r#"impl core::fmt::Display for Foo {
           fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
             let text = include_str!("templates/foo.html");
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
        .unindent()
    );
  }

  #[test]
  #[cfg(feature = "axum")]
  fn axum_guess_default() {
    assert_eq!(
      impls(&Ident::new("Foo", Span::call_site()), "templates/foo", ""),
      r#"impl core::fmt::Display for Foo {
           fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
             let text = include_str!("templates/foo");
             Ok(())
           }
         }
         
         impl axum::response::IntoResponse for Foo {
           fn into_response(self) -> axum::response::Response<axum::body::BoxBody> {
             axum::response::Response::builder()
               .header(axum::http::header::CONTENT_TYPE, "text/plain")
               .body(axum::body::Full::from(self.to_string()))
               .unwrap()
               .into_response()
           }
         }"#
        .unindent()
    );
  }
}
