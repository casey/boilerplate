use super::*;

pub(crate) struct Template {
  pub(crate) escape: bool,
  pub(crate) generics: Generics,
  pub(crate) ident: Ident,
  pub(crate) mime: Mime,
  pub(crate) source: Source,
}

impl Template {
  pub(crate) fn impls(self) -> TokenStream {
    let display_impl = self.boilerplate_impl();

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

  fn boilerplate_impl(&self) -> TokenStream {
    let ident = &self.ident;
    let source = &self.source;
    let text = source.text();

    let (body, template, tokens) = body(&text, self.escape, false);

    let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

    let tokens = if cfg!(feature = "reload") {
      let tokens = tokens
        .into_iter()
        .map(|token| match token {
          Token::Code { contents } => quote!(boilerplate::Token::Code { contents: #contents }),
          Token::CodeLine { closed, contents } => {
            quote!(boilerplate::Token::CodeLine { closed: #closed, contents: #contents })
          }
          Token::Interpolation { contents } => {
            quote!(boilerplate::Token::Interpolation { contents: #contents })
          }
          Token::InterpolationLine { contents, closed } => {
            quote!(boilerplate::Token::InterpolationLine { closed: #closed, contents: #contents })
          }
          Token::Text { contents, index } => quote!(boilerplate::Token::Text {
            contents: #contents,
            index: #index
          }),
        })
        .collect::<Vec<TokenStream>>();

      Some(quote! {
        const TOKENS: &'static [boilerplate::Token<'static>] = &[ #(#tokens),* ];
      })
    } else {
      None
    };

    quote! {
      impl #impl_generics boilerplate::Boilerplate for #ident #ty_generics #where_clause {
        const TEXT: &'static [&'static str] = &[ #(#template),* ];

        #tokens

        fn boilerplate(
          &self,
          boilerplate_text: &[impl AsRef<str>],
          boilerplate_output: &mut core::fmt::Formatter,
        ) -> core::fmt::Result {
          use core::fmt::Write;
          #body
          Ok(())
        }
      }

      impl #impl_generics core::fmt::Display for #ident #ty_generics #where_clause {
        fn fmt(&self, boilerplate_output: &mut core::fmt::Formatter) -> core::fmt::Result {
          <Self as boilerplate::Boilerplate>::boilerplate(
            self,
            <Self as boilerplate::Boilerplate>::TEXT,
            boilerplate_output,
          )
        }
      }
    }
  }

  fn axum_into_response_impl(&self) -> TokenStream {
    let ident = &self.ident;
    let content_type = LitStr::new(self.mime.as_ref(), Span::call_site());
    let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

    quote! {
      impl #impl_generics axum::response::IntoResponse for #ident #ty_generics #where_clause {
        fn into_response(self) -> axum::response::Response {
          (
            [(axum::http::header::CONTENT_TYPE, #content_type)],
            self.to_string(),
          ).into_response()
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq, proc_macro2::Span};

  #[test]
  fn boilerplate_impl() {
    let tokens = if cfg!(feature = "reload") {
      Some(quote! {
        const TOKENS: &'static [boilerplate::Token<'static>] = &[];
      })
    } else {
      None
    };

    assert_eq!(
      Template {
        ident: Ident::new("Foo", Span::call_site()),
        source: Source::Literal(LitStr::new("", Span::call_site())),
        mime: mime::TEXT_PLAIN,
        escape: false,
        generics: Generics::default(),
      }
      .boilerplate_impl()
      .to_string(),
      quote!(
        impl boilerplate::Boilerplate for Foo {
            const TEXT: &'static [&'static str] = &[];

            #tokens

            fn boilerplate(
              &self,
              boilerplate_text: &[impl AsRef<str>],
              boilerplate_output: &mut core::fmt::Formatter,
            ) -> core::fmt::Result {
              use core::fmt::Write;
              Ok(())
            }
        }

        impl core::fmt::Display for Foo {
          fn fmt(&self, boilerplate_output: &mut core::fmt::Formatter) -> core::fmt::Result {
            <Self as boilerplate::Boilerplate>::boilerplate(
              self,
              <Self as boilerplate::Boilerplate>::TEXT,
              boilerplate_output,
            )
          }
        }
      )
      .to_string()
    );
  }

  fn assert_display_body_eq(template: &str, expected: TokenStream) {
    let (actual, _template, _tokens) = body(template, false, false);
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
    assert_display_body_eq(
      "{{ true }}",
      quote!(write!(boilerplate_output, "{}", true)?;),
    );
  }

  #[test]
  fn iteration() {
    assert_display_body_eq(
      "{% for i in 0..10 { %}{{ i }}{% } %}",
      quote!(for i in 0..10 {
        write!(boilerplate_output, "{}", i)?;
      }),
    );
  }

  #[test]
  fn non_trailing_text() {
    assert_display_body_eq(
      "foo {{ true }}",
      quote!(
        boilerplate_output.write_str(boilerplate_text[0].as_ref())?;
        write!(boilerplate_output, "{}", true)?;
      ),
    );
  }

  #[test]
  fn trailing_text() {
    assert_display_body_eq(
      "foo",
      quote!(boilerplate_output.write_str(boilerplate_text[0].as_ref())?;),
    );
  }

  #[test]
  fn axum_into_response_impl() {
    assert_eq!(
      Template {
        ident: Ident::new("Foo", Span::call_site()),
        source: Source::Literal(LitStr::new("", Span::call_site())),
        mime: mime::TEXT_PLAIN,
        escape: false,
        generics: Generics::default(),
      }
      .axum_into_response_impl()
      .to_string(),
      quote!(
        impl axum::response::IntoResponse for Foo {
          fn into_response(self) -> axum::response::Response {
            (
              [(axum::http::header::CONTENT_TYPE, "text/plain")],
              self.to_string(),
            ).into_response()
          }
        }
      )
      .to_string()
    );
  }
}
