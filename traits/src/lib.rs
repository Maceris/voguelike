extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn create_action(item: TokenStream) -> TokenStream {
    if item.is_empty() {
        panic!("Type name expected");
    }

    let name = item.to_string();

    let source = format!("#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct {name};
    ");

    source.parse().unwrap()
}
