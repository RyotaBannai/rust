use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TypeName)]
pub fn derive_typename(input: TokenStream) -> TokenStream {
    // parse_macro_input は TokenStream をパースするマクロ
    // as はキャストではなく、parse_macro_input の構文
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident; // ここで型名（struct 名）を返す

    // generates 'proc_macro2::TokenStream'
    let gen = quote! {
        impl ::typename::TypeNameTrait for #name {
            fn type_name(&self) -> &str{
                stringify!(#name) // 引数を文字列にするマクロ
            }
        }
    };
    // converts 'proc_macro2::TokenStream' to proc_macro::TokenStream
    gen.into()
}
