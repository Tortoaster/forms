extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
    Generics, Type,
};

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    impl_component(ast).into()
}

fn impl_component(ast: DeriveInput) -> TokenStream2 {
    match ast.data {
        Data::Struct(data_struct) => impl_component_struct(ast.ident, data_struct, ast.generics),
        Data::Enum(data_enum) => impl_component_enum(ast.ident, data_enum, ast.generics),
        Data::Union(_) => panic!("deriving Component is not implemented for unions"),
    }
}

fn impl_component_struct(
    ident: Ident,
    data_struct: DataStruct,
    generics: Generics,
) -> TokenStream2 {
    match data_struct.fields {
        Fields::Named(fields_named) => impl_component_named(ident, fields_named, generics),
        Fields::Unnamed(fields_unnamed) => impl_component_unnamed(ident, fields_unnamed, generics),
        Fields::Unit => impl_component_unit(ident, generics),
    }
}

fn impl_component_named(
    ident: Ident,
    fields_named: FieldsNamed,
    generics: Generics,
) -> TokenStream2 {
    let (idents, types): (Vec<Option<Ident>>, Vec<Type>) = fields_named
        .named
        .into_iter()
        .map(|field| (field.ident, field.ty))
        .unzip();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics Component for #ident #ty_generics #where_clause {
            fn view(&self) -> Form {
                Form::Compound(vec![
                    #(self.#idents.view().with_hint(stringify!(#idents).to_case(Case::Title)),)*
                ]).into()
            }

            fn enter() -> Form {
                Form::Compound(vec![
                    #(#types::enter().with_hint(stringify!(#idents).to_case(Case::Title)),)*
                ]).into()
            }

            fn update(&self) -> Form {
                Form::Compound(vec![
                    #(self.#idents.update().with_hint(stringify!(#idents).to_case(Case::Title)),)*
                ]).into()
            }
        }
    }
}

fn impl_component_unnamed(
    name: Ident,
    fields_unnamed: FieldsUnnamed,
    generics: Generics,
) -> TokenStream2 {
    let views: TokenStream2 = (0..fields_unnamed.unnamed.len())
        .map(syn::Index::from)
        .flat_map(|index| {
            quote! {
                format!("{}", self.#index.view())
            }
        })
        .collect();

    let enters: TokenStream2 = fields_unnamed
        .unnamed
        .iter()
        .flat_map(|field| {
            let ty = &field.ty;
            quote! {
                format!("{}", #ty::enter())
            }
        })
        .collect();

    let updates: TokenStream2 = (0..fields_unnamed.unnamed.len())
        .map(syn::Index::from)
        .flat_map(|index| {
            quote! {
                format!("{}", self.#index.update())
            }
        })
        .collect();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics Component for #name #ty_generics #where_clause {
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
    }
}

fn impl_component_unit(ident: Ident, generics: Generics) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics Component for #ident #ty_generics #where_clause {
            fn view(&self) -> String {
                format!("<div class=\"component\"><div class=\"title\">{}</div></div>", stringify!(#ident))
            }

            fn enter() -> String {
                format!("<div class=\"component\"><div class=\"title\">{}</div></div>", stringify!(#ident))
            }

            fn update(&self) -> String {
                format!("<div class=\"component\"><div class=\"title\">{}</div></div>", stringify!(#ident))
            }
        }
    }
}

fn impl_component_enum(ident: Ident, data_enum: DataEnum, generics: Generics) -> TokenStream2 {
    let matches: TokenStream2 = data_enum.variants.iter().flat_map(|variant| {
        let variant_name = &variant.ident;

        let variant_fields = match &variant.fields {
            Fields::Named(_) => quote_spanned! {variant.fields.span()=> {..} },
            Fields::Unnamed(_) => quote_spanned! {variant.fields.span()=> (..) },
            Fields::Unit => quote_spanned! { variant.fields.span()=> },
        };

        quote_spanned! {variant.span()=>
            #ident::#variant_name #variant_fields => format!("<div class=\"component\"><div class=\"title\">{}</div><div class=\"subtitle\">{}</div><div class=\"content\"></div></div>", stringify!(#ident), stringify!(#variant_name)),
        }
    }).collect();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics Component for #ident #ty_generics #where_clause {
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
    }
}
