use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DemoDerive)]
pub fn demo_derive(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl DemoTrait for #name {
            fn demo(&self) {
                println!("Demo for {}", stringify!(#name));
            }
        }
    };
    gen.into()
}