use super::*;

pub(crate) struct Template {
  pub(crate) axum: Option<bool>,
  pub(crate) escape: bool,
  pub(crate) generics: Generics,
  pub(crate) ident: Ident,
  pub(crate) mime: Mime,
  pub(crate) source: Source,
}

impl Template {
  pub(crate) fn impls(self) -> TokenStream {
    let display_impl = self.display_impl();

    let axum_into_response_impl = if self.axum.unwrap_or(cfg!(feature = "axum")) {
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
    let src = source.src();

    let Implementation { body, text, tokens } = Implementation::parse(&src, self.escape, false);

    let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

    let tokens = if cfg!(feature = "reload") {
      let tokens = tokens
        .into_iter()
        .map(|token| match token {
          Token::Code { contents } => quote!(::boilerplate::Token::Code { contents: #contents }),
          Token::CodeLine { closed, contents } => {
            quote!(::boilerplate::Token::CodeLine { closed: #closed, contents: #contents })
          }
          Token::Interpolation { contents } => {
            quote!(::boilerplate::Token::Interpolation { contents: #contents })
          }
          Token::InterpolationLine { contents, closed } => {
            quote!(::boilerplate::Token::InterpolationLine { closed: #closed, contents: #contents })
          }
          Token::Text { contents, index } => quote!(::boilerplate::Token::Text {
            contents: #contents,
            index: #index
          }),
        })
        .collect::<Vec<TokenStream>>();

      Some(quote! {
        const TOKENS: &'static [::boilerplate::Token<'static>] = &[ #(#tokens),* ];
      })
    } else {
      None
    };

    let path = if cfg!(feature = "reload") {
      if let Source::Path(path) = &self.source {
        Some(quote!(const PATH: Option<&'static str> = Some(#path);))
      } else {
        Some(quote!(
          const PATH: Option<&'static str> = None;
        ))
      }
    } else {
      None
    };

    quote! {
      impl #impl_generics ::boilerplate::Boilerplate for #ident #ty_generics #where_clause {
        const TEXT: &'static [&'static str] = &[ #(#text),* ];

        #tokens

        #path

        fn boilerplate(
          &self,
          boilerplate_text: &[impl ::core::convert::AsRef<str>],
          boilerplate_output: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
          use ::core::fmt::Write;
          use ::boilerplate::Escape;
          #body
          Ok(())
        }
      }

      impl #impl_generics ::core::fmt::Display for #ident #ty_generics #where_clause {
        fn fmt(&self, boilerplate_output: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
          <Self as ::boilerplate::Boilerplate>::boilerplate(
            self,
            <Self as ::boilerplate::Boilerplate>::TEXT,
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
      impl #impl_generics ::axum::response::IntoResponse for #ident #ty_generics #where_clause {
        fn into_response(self) -> ::axum::response::Response {
          extern crate alloc;
          use alloc::string::ToString;
          (
            [(::axum::http::header::CONTENT_TYPE, #content_type)],
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
  fn display_impl() {
    let tokens = if cfg!(feature = "reload") {
      Some(quote! {
        const TOKENS: &'static [::boilerplate::Token<'static>] = &[
          ::boilerplate::Token::Text { contents: "", index: 0usize }
        ];
      })
    } else {
      None
    };

    let path = if cfg!(feature = "reload") {
      Some(quote!(
        const PATH: Option<&'static str> = None;
      ))
    } else {
      None
    };

    let text = if cfg!(feature = "reload") {
      Some("")
    } else {
      None
    };

    let body = if cfg!(feature = "reload") {
      Some(quote!(boilerplate_output.write_str(boilerplate_text[0].as_ref())?;))
    } else {
      None
    };

    assert_eq!(
      Template {
        axum: None,
        escape: false,
        generics: Generics::default(),
        ident: Ident::new("Foo", Span::call_site()),
        mime: mime::TEXT_PLAIN,
        source: Source::Literal(LitStr::new("", Span::call_site())),
      }
      .display_impl()
      .to_string(),
      quote!(
        impl ::boilerplate::Boilerplate for Foo {
            const TEXT: &'static [&'static str] = &[#text];

            #tokens

            #path

            fn boilerplate(
              &self,
              boilerplate_text: &[impl ::core::convert::AsRef<str>],
              boilerplate_output: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
              use ::core::fmt::Write;
              use ::boilerplate::Escape;
              #body
              Ok(())
            }
        }

        impl ::core::fmt::Display for Foo {
          fn fmt(&self, boilerplate_output: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            <Self as ::boilerplate::Boilerplate>::boilerplate(
              self,
              <Self as ::boilerplate::Boilerplate>::TEXT,
              boilerplate_output,
            )
          }
        }
      )
      .to_string()
    );
  }

  fn assert_display_body_eq(template: &str, expected: TokenStream) {
    assert_eq!(
      Implementation::parse(template, false, false)
        .body
        .to_string(),
      expected.to_string(),
    );
  }

  #[test]
  fn empty() {
    if cfg!(feature = "reload") {
      assert_display_body_eq(
        "",
        quote!(
          boilerplate_output.write_str(boilerplate_text[0].as_ref())?;
        ),
      );
    } else {
      assert_display_body_eq("", quote!());
    }
  }

  #[test]
  fn code() {
    if cfg!(feature = "reload") {
      assert_display_body_eq(
        "{% (); %}",
        quote!(
          boilerplate_output.write_str(boilerplate_text[0].as_ref())?;
          ();
          boilerplate_output.write_str(boilerplate_text[1].as_ref())?;
        ),
      );
    } else {
      assert_display_body_eq(
        "{% (); %}",
        quote!(
          ();
        ),
      );
    }
  }

  #[test]
  fn interpolation() {
    if cfg!(feature = "reload") {
      assert_display_body_eq(
        "{{ true }}",
        quote!(
          boilerplate_output.write_str(boilerplate_text[0].as_ref())?;
          write!(boilerplate_output, "{}", true)?;
          boilerplate_output.write_str(boilerplate_text[1].as_ref())?;
        ),
      );
    } else {
      assert_display_body_eq(
        "{{ true }}",
        quote!(
          write!(boilerplate_output, "{}", true)?;
        ),
      );
    }
  }

  #[test]
  fn iteration() {
    if cfg!(feature = "reload") {
      assert_display_body_eq(
        "{% for i in 0..10 { %}{{ i }}{% } %}",
        quote!(
          boilerplate_output.write_str(boilerplate_text[0].as_ref())?;
          for i in 0..10 {
            boilerplate_output.write_str(boilerplate_text[1].as_ref())?;
            write!(boilerplate_output, "{}", i)?;
            boilerplate_output.write_str(boilerplate_text[2].as_ref())?;
          }
          boilerplate_output.write_str(boilerplate_text[3].as_ref())?;
        ),
      );
    } else {
      assert_display_body_eq(
        "{% for i in 0..10 { %}{{ i }}{% } %}",
        quote!(for i in 0..10 {
          write!(boilerplate_output, "{}", i)?;
        }),
      );
    }
  }

  #[test]
  fn non_trailing_text() {
    if cfg!(feature = "reload") {
      assert_display_body_eq(
        "foo {{ true }}",
        quote!(
          boilerplate_output.write_str(boilerplate_text[0].as_ref())?;
          write!(boilerplate_output, "{}", true)?;
          boilerplate_output.write_str(boilerplate_text[1].as_ref())?;
        ),
      );
    } else {
      assert_display_body_eq(
        "foo {{ true }}",
        quote!(
          boilerplate_output.write_str(boilerplate_text[0].as_ref())?;
          write!(boilerplate_output, "{}", true)?;
        ),
      );
    }
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
        axum: Some(true),
        escape: false,
        generics: Generics::default(),
        ident: Ident::new("Foo", Span::call_site()),
        mime: mime::TEXT_PLAIN,
        source: Source::Literal(LitStr::new("", Span::call_site())),
      }
      .axum_into_response_impl()
      .to_string(),
      quote!(
        impl ::axum::response::IntoResponse for Foo {
          fn into_response(self) -> ::axum::response::Response {
            extern crate alloc;
            use alloc::string::ToString;

            (
              [(::axum::http::header::CONTENT_TYPE, "text/plain")],
              self.to_string(),
            ).into_response()
          }
        }
      )
      .to_string()
    );
  }
}
