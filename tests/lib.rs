use boilerplate::Display;

#[test]
fn code() {
  #[derive(Display)]
  struct CodeHtml {}
  assert_eq!(CodeHtml {}.to_string(), "0123456789\n");
}

#[test]
fn code_line() {
  #[derive(Display)]
  struct CodeLineHtml {}
  assert_eq!(
    CodeLineHtml {}.to_string(),
    "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n"
  );
}

#[test]
fn empty() {
  #[derive(Display)]
  struct EmptyHtml {}
  assert_eq!(EmptyHtml {}.to_string(), "");
}

#[test]
fn interpolation() {
  #[derive(Display)]
  struct InterpolationHtml {}
  assert_eq!(InterpolationHtml {}.to_string(), "true bar false\n");
}

#[test]
fn interpolation_line() {
  #[derive(Display)]
  struct InterpolationLineHtml {}
  assert_eq!(InterpolationLineHtml {}.to_string(), "true\nbar\nfalse\n");
}

#[test]
fn interpolation_line_multiple_statements() {
  #[derive(Display)]
  struct InterpolationLineMultipleStatementsHtml {}
  assert_eq!(
    InterpolationLineMultipleStatementsHtml {}.to_string(),
    "true\n"
  );
}

#[test]
fn interpolation_multiple_statements() {
  #[derive(Display)]
  struct InterpolationMultipleStatementsHtml {}
  assert_eq!(InterpolationMultipleStatementsHtml {}.to_string(), "true\n");
}

#[test]
fn match_html() {
  #[derive(Display)]
  struct MatchHtml {
    item: Option<&'static str>,
  }
  assert_eq!(
    MatchHtml { item: Some("foo") }.to_string(),
    "Found literal foo\n"
  );
  assert_eq!(MatchHtml { item: Some("bar") }.to_string(), "Found bar\n");
  assert_eq!(MatchHtml { item: None }.to_string(), "");
}

#[test]
fn trivial() {
  #[derive(Display)]
  struct TrivialHtml {}
  assert_eq!(TrivialHtml {}.to_string(), "foo\n");
}
