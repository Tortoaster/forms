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
            match &data_struct.fields {
                Fields::Named(fields_named) => {
                    let views: TokenStream2 = fields_named.named.iter().flat_map(|field| {
                        let name = &field.ident;
                        quote_spanned! {field.span()=>
                            format!("{}: {}", stringify!(#name), self.#name.view()),
                        }
                    }).collect();

                    let enters: TokenStream2 = fields_named.named.iter().flat_map(|field| {
                        let name = &field.ident;
                        let ty = &field.ty;
                        quote_spanned! {field.span()=>
                            format!("{}: {}", stringify!(#name), #ty::enter()),
                        }
                    }).collect();

                    let updates: TokenStream2 = fields_named.named.iter().flat_map(|field| {
                        let name = &field.ident;
                        quote_spanned! {field.span()=>
                            format!("{}: {}", stringify!(#name), self.#name.update()),
                        }
                    }).collect();

                    let gen = quote! {
                        impl Component for #name {
                            fn view(&self) -> String {
                                let fields = vec![
                                    #views
                                ];
                                format!("<div class=\"component\"><div class=\"title\">{}</div><div class=\"content\">{}</div></div>", stringify!(#name), fields.join("<hr/>"))
                            }

                            fn enter() -> String {
                                let fields = vec![
                                    #enters
                                ];
                                format!("<div class=\"component\"><div class=\"title\">{}</div><div class=\"content\">{}</div></div>", stringify!(#name), fields.join("<hr/>"))
                            }

                            fn update(&self) -> String {
                                let fields = vec![
                                    #updates
                                ];
                                format!("<div class=\"component\"><div class=\"title\">{}</div><div class=\"content\">{}</div></div>", stringify!(#name), fields.join("<hr/>"))
                            }
                        }
                    };

                    gen.into()
                }
                Fields::Unnamed(fields_unnamed) => {
                    let views: TokenStream2 = (0..fields_unnamed.unnamed.len()).map(syn::Index::from).flat_map(|index| {
                        quote! {
                            format!("{}", self.#index.view())
                        }
                    }).collect();

                    let enters: TokenStream2 = fields_unnamed.unnamed.iter().flat_map(|field| {
                        let ty = &field.ty;
                        quote! {
                            format!("{}", #ty::enter())
                        }
                    }).collect();

                    let updates: TokenStream2 = (0..fields_unnamed.unnamed.len()).map(syn::Index::from).flat_map(|index| {
                        quote! {
                            format!("{}", self.#index.update())
                        }
                    }).collect();

                    let gen = quote! {
                        impl Component for #name {
                            fn view(&self) -> String {
                                let fields = vec![
                                    #views
                                ];
                                format!("<div class=\"component\"><div class=\"title\">{}</div><div class=\"content\">{}</div></div>", stringify!(#name), fields.join("<hr/>"))
                            }

                            fn enter() -> String {
                                let fields = vec![
                                    #enters
                                ];
                                format!("<div class=\"component\"><div class=\"title\">{}</div><div class=\"content\">{}</div></div>", stringify!(#name), fields.join("<hr/>"))
                            }

                            fn update(&self) -> String {
                                let fields = vec![
                                    #updates
                                ];
                                format!("<div class=\"component\"><div class=\"title\">{}</div><div class=\"content\">{}</div></div>", stringify!(#name), fields.join("<hr/>"))
                            }
                        }
                    };

                    gen.into()
                }
                Fields::Unit => {
                    let gen = quote! {
                        impl Component for #name {
                            fn view(&self) -> String {
                                format!("<div class=\"component\"><div class=\"title\">{}</div></div>", stringify!(#name))
                            }

                            fn enter() -> String {
                                format!("<div class=\"component\"><div class=\"title\">{}</div></div>", stringify!(#name))
                            }

                            fn update(&self) -> String {
                                format!("<div class=\"component\"><div class=\"title\">{}</div></div>", stringify!(#name))
                            }
                        }
                    };

                    gen.into()
                }
            }
        }
        Data::Enum(data_enum) => {
            let matches: TokenStream2 = data_enum.variants.iter().flat_map(|variant| {
                let variant_name = &variant.ident;

                let variant_fields = match &variant.fields {
                    Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
                    Fields::Unit => quote_spanned! { variant.span()=> },
                    Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
                };

                quote_spanned! {variant.span()=>
                    #name::#variant_name #variant_fields => format!("<div class=\"component\"><div class=\"title\">{}</div><div class=\"subtitle\">{}</div><div class=\"content\"></div></div>", stringify!(#name), stringify!(#variant_name)),
                }
            }).collect();

            let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

            let gen = quote! {
                impl #impl_generics Component for #name #ty_generics #where_clause {
                    fn view(&self) -> String {
                        match self {
                            #matches
                        }
                    }

                    fn enter() -> String {

                    }

                    fn update(&self) -> String {

                    }
                }
            };

            gen.into()
        }
        Data::Union(_) => panic!("deriving Component is not implemented for unions")
    }
}
