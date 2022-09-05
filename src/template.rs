use super::*;

pub(crate) struct Template {
  pub(crate) ident: Ident,
  pub(crate) source: Source,
  pub(crate) text: String,
}

impl Template {
  pub(crate) fn impls(self) -> TokenStream {
    let display_impl = self.display_impl();

    let axum_into_response_impl = if cfg!(feature = "axum") {
      Some(self.axum_into_response_impl())
    } else {
      None
    };

    quote! {
      #display_impl
      #axum_into_response_impl
    }
  }

  fn display_impl(&self) -> TokenStream {
    let ident = &self.ident;
    let source = &self.source;
    let body = self.body();
    quote! {
      impl core::fmt::Display for #ident {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
          let text = #source;
          #body
          Ok(())
        }
      }
    }
  }

  fn body(&self) -> TokenStream {
    let mut lines = Vec::new();
    let mut i = 0;
    let mut j = 0;
    loop {
      let rest = &self.text[j..];

      let block = Block::starting_at(rest);

      if i < j && block.is_some() {
        lines.push(format!("f.write_str(&text[{}..{}])? ;", i, j));
      }

      if i < j && j == self.text.len() {
        lines.push(format!("f.write_str(&text[{}..])? ;", i));
      }

      if j == self.text.len() {
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

    lines.join("").parse().unwrap()
  }

  fn axum_into_response_impl(&self) -> TokenStream {
    let ident = &self.ident;
    let content_type = LitStr::new(self.source.mime().as_ref(), Span::call_site());
    quote!(
      impl axum::response::IntoResponse for #ident {
        fn into_response(self) -> axum::response::Response<axum::body::BoxBody> {
          axum::response::Response::builder()
            .header(axum::http::header::CONTENT_TYPE, #content_type)
          .body(axum::body::Full::from(self.to_string()))
          .unwrap()
          .into_response()
        }
      }
    )
  }
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq, proc_macro2::Span};

  #[test]
  fn display_impl() {
    assert_eq!(
      Template {
        ident: Ident::new("Foo", Span::call_site()),
        text: "".into(),
        source: Source::Path("templates/foo".into()),
      }
      .display_impl()
      .to_string(),
      quote!(
        impl core::fmt::Display for Foo {
          fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            let text = include_str!("templates/foo");
            Ok(())
          }
        }
      )
      .to_string()
    );
  }

  fn assert_display_body_eq(template: &str, expected: TokenStream) {
    let actual = Template {
      ident: Ident::new("Foo", Span::call_site()),
      text: template.into(),
      source: Source::Path("templates/foo".into()),
    }
    .body();

    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn empty() {
    assert_display_body_eq("", quote!());
  }

  #[test]
  fn code() {
    assert_display_body_eq("{% () %}", quote!(()));
  }

  #[test]
  fn interpolation() {
    assert_display_body_eq("{{ true }}", quote!(write!(f, "{}", true)?;));
  }

  #[test]
  fn iteration() {
    assert_display_body_eq(
      "{% for i in 0..10 { %}{{ i }}{% } %}",
      quote!(for i in 0..10 {
        write!(f, "{}", i)?;
      }),
    );
  }

  #[test]
  fn non_trailing_text() {
    assert_display_body_eq(
      "foo {{ true }}",
      quote!(
        f.write_str(&text[0..4])?;
        write!(f, "{}", true)?;
      ),
    );
  }

  #[test]
  fn trailing_text() {
    assert_display_body_eq("foo", quote!(f.write_str(&text[0..])?;));
  }

  fn assert_axum_into_response_impl_eq(path: &str, expected: TokenStream) {
    let actual = Template {
      ident: Ident::new("Foo", Span::call_site()),
      text: "".into(),
      source: Source::Path(path.into()),
    }
    .axum_into_response_impl();

    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn axum_guess_from_path() {
    assert_axum_into_response_impl_eq(
      "foo.html",
      quote!(
        impl axum::response::IntoResponse for Foo {
          fn into_response(self) -> axum::response::Response<axum::body::BoxBody> {
            axum::response::Response::builder()
              .header(axum::http::header::CONTENT_TYPE, "text/html")
              .body(axum::body::Full::from(self.to_string()))
              .unwrap()
              .into_response()
          }
        }
      ),
    );
  }

  #[test]
  fn axum_guess_default() {
    assert_axum_into_response_impl_eq(
      "foo",
      quote!(
        impl axum::response::IntoResponse for Foo {
          fn into_response(self) -> axum::response::Response<axum::body::BoxBody> {
            axum::response::Response::builder()
              .header(axum::http::header::CONTENT_TYPE, "text/plain")
              .body(axum::body::Full::from(self.to_string()))
              .unwrap()
              .into_response()
          }
        }
      ),
    );
  }
}
