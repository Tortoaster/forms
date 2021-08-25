extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, Fields};

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_component(&ast)
}

fn impl_component(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let content = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let names = fields_named.named.iter().map(|f| &f.ident);
                quote! {
                    let content = vec![
                        #(format!("{}: {}", stringify!(#names), self.#names.view())),*
                    ];
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let nums = (0..fields_unnamed.unnamed.len()).map(syn::Index::from);
                quote! {
                    let content = vec![
                        #(format!("{}", self.#nums.view())),*
                    ];
                }
            }
            Fields::Unit => {
                quote! {
                    let content = Vec::new();
                }
            }
        }
        Data::Enum(_) => panic!("deriving Component is not yet implemented for enums"),
        Data::Union(_) => panic!("deriving Component is not implemented for unions")
    };

    let gen = quote! {
        impl Component for #name {
            fn view(&self) -> String {
                #content
                format!("<div><h5>{}</h5>{}</div>", stringify!(#name), content.join("<br/>"))
            }
        }
    };

    gen.into()
}
