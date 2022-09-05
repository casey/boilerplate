use super::*;

#[derive(FromDeriveInput)]
#[darling(attributes(display))]
pub(crate) struct Display {
  ident: Ident,
  text: Option<LitStr>,
}

impl Display {
  pub(crate) fn impls(self) -> TokenStream {
    let source = match self.text {
      Some(text) => Source::Literal(text),
      None => {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
          .expect("Failed to get `CARGO_MANIFEST_DIR` environment variable");

        let path = Path::new(&manifest_dir)
          .join("templates")
          .join(filename_from_ident(&self.ident.to_string()));

        let path = path.to_str().unwrap_or_else(|| {
          panic!(
            "Path to template `{}` was not valid unicode",
            path.display()
          )
        });

        Source::Path(path.into())
      }
    };

    Template {
      ident: self.ident,
      source,
    }
    .impls()
  }
}
