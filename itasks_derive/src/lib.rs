extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{Data, DeriveInput, Fields, parse_macro_input};
use syn::spanned::Spanned;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    match &ast.data {
        Data::Struct(data_struct) => {
            let content = match &data_struct.fields {
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
        Data::Enum(data_enum) => {
            let mut matches = TokenStream2::new();

            for variant in &data_enum.variants {
                let variant_name = &variant.ident;

                let variant_fields = match &variant.fields {
                    Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
                    Fields::Unit => quote_spanned! { variant.span()=> },
                    Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
                };

                matches.extend(quote_spanned! {variant.span()=>
                    #name::#variant_name #variant_fields => format!("<div><h5>{}</h5><h6>{}</h6></div>", stringify!(#name), stringify!(#variant_name)),
                });
            }

            let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

            let gen = quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    fn view(&self) -> String {
                        match self {
                            #matches
                        }
                    }
                }
            };

            gen.into()
        }
        Data::Union(_) => panic!("deriving Component is not implemented for unions")
    }
}
