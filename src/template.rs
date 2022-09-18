use super::*;

pub(crate) struct Template {
  pub(crate) escape: bool,
  pub(crate) ident: Ident,
  pub(crate) mime: Mime,
  pub(crate) source: Source,
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
          use core::fmt::Write;
          let text = #source;
          #body
          Ok(())
        }
      }
    }
  }

  fn body(&self) -> TokenStream {
    let text = self.source.text();
    let mut lines = Vec::new();
    let mut i = 0;
    let mut j = 0;
    loop {
      let rest = &text[j..];

      let block = Block::starting_at(rest, self.escape);

      if i < j && block.is_some() {
        lines.push(format!("f.write_str(&text[{}..{}])? ;", i, j));
      }

      if i < j && j == text.len() {
        lines.push(format!("f.write_str(&text[{}..])? ;", i));
      }

      if j == text.len() {
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
    let content_type = LitStr::new(self.mime.as_ref(), Span::call_site());
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
        source: Source::Literal(LitStr::new("", Span::call_site())),
        mime: mime::TEXT_PLAIN,
        escape: false,
      }
      .display_impl()
      .to_string(),
      quote!(
        impl core::fmt::Display for Foo {
          fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            use core::fmt::Write;
            let text = "";
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
      source: Source::Literal(LitStr::new(template, Span::call_site())),
      mime: mime::TEXT_PLAIN,
      escape: false,
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

  #[test]
  fn axum_into_response_impl() {
    assert_eq!(
      Template {
        ident: Ident::new("Foo", Span::call_site()),
        source: Source::Literal(LitStr::new("", Span::call_site())),
        mime: mime::TEXT_PLAIN,
        escape: false,
      }
      .axum_into_response_impl()
      .to_string(),
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
      )
      .to_string()
    );
  }
}
