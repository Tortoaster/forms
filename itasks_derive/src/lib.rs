extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
    Generics, Index, Type,
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
    let ([view, enter, update], idents) = forms_fields(ident.clone(), data_struct.fields.clone());

    let (values, idents): (Vec<TokenStream2>, Vec<Ident>) = idents
        .into_iter()
        .enumerate()
        .map(|(index, ident)| match ident {
            None => {
                let i = Index::from(index);
                (quote! { &self.#i }, unnamed_ident(index, &ident))
            }
            Some(ident) => (quote! { &self.#ident }, ident),
        })
        .unzip();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics Component for #ident #ty_generics #where_clause {
            fn view(&self) -> Form {
                #(let #idents = #values;)*
                #view
            }

            fn enter() -> Form {
                #enter
            }

            fn update(&self) -> Form {
                #(let #idents = #values;)*
                #update
            }
        }
    }
}

fn impl_component_enum(enum_ident: Ident, data_enum: DataEnum, generics: Generics) -> TokenStream2 {
    let (idents, x): (Vec<_>, Vec<_>) = data_enum
        .variants
        .into_iter()
        .map(|variant| {
            let variant_ident = variant.ident;
            let fields = fields_idents(&variant.fields);
            let ([view, enter, update], _) = forms_fields(variant_ident.clone(), variant.fields);
            let arm = quote! { #enum_ident::#variant_ident#fields };
            (variant_ident, (arm, (view, (enter, update))))
        })
        .unzip();
    let (arms, x): (Vec<_>, Vec<_>) = x.into_iter().unzip();
    let (views, x): (Vec<_>, Vec<_>) = x.into_iter().unzip();
    let (enters, updates): (Vec<_>, Vec<_>) = x.into_iter().unzip();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics Component for #enum_ident #ty_generics #where_clause {
            fn view(&self) -> Form {
                match self {
                    #(#arms => #views,)*
                }
            }

            fn enter() -> Form {
                let choices = vec![
                    #((stringify!(#idents).to_case(Case::Title), #enters),)*
                ];
                let choice = choices.get(0).map(|choice| choice.0.clone()).unwrap_or_default();
                Form::new(vec![Input::new(InputValue::Choice(choices.into_iter().collect(), choice))])
                    .with_title(stringify!(#enum_ident).to_case(Case::Title))
            }

            fn update(&self) -> Form {
                match self {
                    #(#arms => #updates,)*
                }
            }
        }
    }
}

fn forms_fields(ident: Ident, fields: Fields) -> ([TokenStream2; 3], Vec<Option<Ident>>) {
    match fields {
        Fields::Named(fields_named) => forms_named(ident, fields_named),
        Fields::Unnamed(fields_unnamed) => forms_unnamed(ident, fields_unnamed),
        Fields::Unit => forms_unit(ident),
    }
}

fn forms_named(ident: Ident, fields_named: FieldsNamed) -> ([TokenStream2; 3], Vec<Option<Ident>>) {
    let (idents, types): (Vec<Option<Ident>>, Vec<Type>) = fields_named
        .named
        .into_iter()
        .map(|field| (field.ident, field.ty))
        .unzip();

    let view = quote! {
        Form::new(vec![
            #(#idents.view().into_iter().collect::<Form>().with_hint(stringify!(#idents).to_case(Case::Title)),)*
        ].into_iter().flatten().flatten().collect()).with_title(stringify!(#ident).to_case(Case::Title)).readonly()
    };

    let enter = quote! {
        Form::new(vec![
            #(#types::enter().into_iter().collect::<Form>().with_hint(stringify!(#idents).to_case(Case::Title)),)*
        ].into_iter().flatten().flatten().collect()).with_title(stringify!(#ident).to_case(Case::Title))
    };

    let update = quote! {
        Form::new(vec![
            #(#idents.update().into_iter().collect::<Form>().with_hint(stringify!(#idents).to_case(Case::Title)),)*
        ].into_iter().flatten().flatten().collect()).with_title(stringify!(#ident).to_case(Case::Title))
    };

    ([view, enter, update], idents)
}

fn forms_unnamed(
    ident: Ident,
    fields_unnamed: FieldsUnnamed,
) -> ([TokenStream2; 3], Vec<Option<Ident>>) {
    let (idents, types): (Vec<Ident>, Vec<Type>) = fields_unnamed
        .unnamed
        .into_iter()
        .enumerate()
        .map(|(index, field)| (unnamed_ident(index, &field.ident), field.ty))
        .unzip();

    let view = quote! {
        Form::new(vec![
            #(#idents.view().into_iter().collect::<Form>(),)*
        ].into_iter().flatten().collect()).with_title(stringify!(#ident).to_case(Case::Title)).readonly()
    };

    let enter = quote! {
        Form::new(vec![
            #(#types::enter().into_iter().collect::<Form>(),)*
        ].into_iter().flatten().collect()).with_title(stringify!(#ident).to_case(Case::Title))
    };

    let update = quote! {
        Form::new(vec![
            #(#idents.update().into_iter().collect::<Form>(),)*
        ].into_iter().flatten().collect()).with_title(stringify!(#ident).to_case(Case::Title))
    };

    let idents = idents.into_iter().map(|_| None).collect();

    ([view, enter, update], idents)
}

fn forms_unit(ident: Ident) -> ([TokenStream2; 3], Vec<Option<Ident>>) {
    let view = quote! {
        Form::new(Vec::new())
            .with_title(stringify!(#ident).to_case(Case::Title))
            .readonly()
    };

    let enter = quote! {
        Form::new(Vec::new())
            .with_title(stringify!(#ident)
            .to_case(Case::Title))
    };

    let update = quote! {
        Form::new(Vec::new())
            .with_title(stringify!(#ident)
            .to_case(Case::Title))
    };

    let idents = Vec::new();

    ([view, enter, update], idents)
}

fn unnamed_ident(index: usize, ident: &Option<Ident>) -> Ident {
    Ident::new(format!("p{}", index).as_str(), ident.span())
}

fn fields_idents(fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(fields_named) => {
            let fields: Vec<_> = fields_named
                .named
                .iter()
                .map(|field| &field.ident)
                .collect();

            quote! { { #(#fields,)* } }
        }
        Fields::Unnamed(fields_unnamed) => {
            let fields: Vec<_> = fields_unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, field)| unnamed_ident(index, &field.ident))
                .collect();

            quote! { (#(#fields,)*) }
        }
        Fields::Unit => quote! {},
    }
}
