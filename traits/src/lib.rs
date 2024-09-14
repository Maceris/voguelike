extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream}, Item};

#[proc_macro]
pub fn create_action(item: TokenStream) -> TokenStream {
    if item.is_empty() {
        panic!("Type name expected");
    }

    // let parsed: Item = syn::parse(item).unwrap();

    // let value = match parsed {
    //     Item::Verbatim(text) => text,
    //     _ => {panic!("Type name expected")}
    // };
    
    let name = item.to_string();

    let source = format!("#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct {name};
        impl Action for {name} {{}}
    ");

    source.parse().unwrap()
}
