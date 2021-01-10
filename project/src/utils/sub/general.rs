pub fn app_to_string<S: Into<String>>(s: S) -> Option<String> {
  Some(s.into())
}

pub fn test_two_string_type() {
  let s1 = app_to_string("str");
  let s2 = app_to_string("str".to_string());
  let s3 = app_to_string(String::from("str"));
  // assert_eq!(s1, s2); // these are the same so assert macro show nothing.
  // assert_eq!(s1, s3); // ok as well.
  assert_eq!(s1, Some("".to_string())) // panic!
                                       // RUST_BACKTRACE=full // for details stack trace
                                       // RUST_BACKTRACE=1
}
