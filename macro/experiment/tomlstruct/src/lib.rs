use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::str::FromStr;

// https://doc.rust-lang.org/reference/procedural-macros.html

// https://rust-lang.github.io/rfcs/0093-remove-format-intl.html
// {{/}} would become escapes for { and }, respectively

#[proc_macro] // 手続きマクロであることを宣言
pub fn tomlstruct(input: TokenStream) -> TokenStream {
  let mut ret = String::from("");
  for token in input {
    match &token {
      TokenTree::Group(x) => {
        let name = get_struct_name(x).unwrap();
        if ret == "" {
          ret = format!("struct {} {{", name);
        } else {
          // be able to add multiple structs
          ret = format!("{}\n}}\nstruct {}{{", ret, name)
        }
      }
      TokenTree::Ident(x) => ret = format!("{}\n  {}", ret, x.to_string()),
      TokenTree::Literal(x) => {
        if x.to_string().starts_with('"') {
          ret = format!("{}: String,", ret);
        } else {
          ret = format!("{}: f64,", ret);
        }
      }
      _ => {}
    }
  }
  ret = format!("{}\n}}", ret);
  println!("{}", &ret);
  // 生成した文字列を TokenStream へ変換
  FromStr::from_str(&ret).unwrap()
}

// tomolstruct!{
// [Hello]
// name = "hello"
// version = 1.0
// }
//
// 構文要素(lexical token)のうち、
// [hello] は Group, name, version は Ident, "Hello", 1.0 は Literal となる

fn get_struct_name(input: &Group) -> Option<String> {
  match input.delimiter() {
    // Bracket で括弧の種類を判別
    Delimiter::Bracket => {
      // stream で括弧内の TokenStream を取得
      for token in input.stream() {
        // Hello を取得
        if let TokenTree::Ident(x) = token {
          return Some(x.to_string());
        }
      }
    }
    _ => (),
  }
  None
}
