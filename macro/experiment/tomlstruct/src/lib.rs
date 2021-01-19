use proc_macro::TokenStream;
// https://doc.rust-lang.org/reference/procedural-macros.html

#[proc_macro] // 手続きマクロであることを宣言
pub fn tomlstruct(input: TokenStream) -> TokenStream {
    dbg!(&input);
    input
}
