extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
    Generics, Index, Type,
};

/// Implements `Component` for a struct or enum.
#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    impl_component(ast).into()
}

/// Implements `Component` for a struct or enum.
fn impl_component(ast: DeriveInput) -> TokenStream2 {
    match ast.data {
        Data::Struct(data_struct) => impl_component_struct(ast.ident, data_struct, ast.generics),
        Data::Enum(data_enum) => impl_component_enum(ast.ident, data_enum, ast.generics),
        Data::Union(_) => panic!("deriving Component is not implemented for unions"),
    }
}

/// Implements `Component` for a struct.
fn impl_component_struct(
    ident: Ident,
    data_struct: DataStruct,
    generics: Generics,
) -> TokenStream2 {
    let idents = fields_idents_struct(&data_struct.fields);
    let [view, enter, update] = forms_fields(&ident, &data_struct.fields);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics Component for #ident #ty_generics #where_clause {
            fn view(&self) -> Form {
                #idents
                #view
            }

            fn enter() -> Form {
                #enter
            }

            fn update(&self) -> Form {
                #idents
                #update
            }
        }
    }
}

/// Implements `Component` for an enum.
fn impl_component_enum(enum_ident: Ident, data_enum: DataEnum, generics: Generics) -> TokenStream2 {
    let (idents, x): (Vec<_>, Vec<_>) = data_enum
        .variants
        .into_iter()
        .map(|variant| {
            let variant_ident = variant.ident;
            let fields = fields_idents_variant(&variant.fields);
            let [view, enter, update] = forms_fields(&variant_ident, &variant.fields);
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

/// Generates form representations for any type of struct or variant.
///
/// Assumes all fields of the struct are already in scope, either with their own identifiers for
/// structs, or with identifiers `p0`, `p1`, ... for tuple structs.
fn forms_fields(ident: &Ident, fields: &Fields) -> [TokenStream2; 3] {
    match fields {
        Fields::Named(fields_named) => forms_struct(ident, fields_named),
        Fields::Unnamed(fields_unnamed) => forms_tuple(ident, fields_unnamed),
        Fields::Unit => forms_unit(ident),
    }
}

/// Generates form representations for a struct or struct variant.
///
/// Assumes all fields of the struct are already in scope with their own identifiers.
fn forms_struct(ident: &Ident, fields_named: &FieldsNamed) -> [TokenStream2; 3] {
    let (idents, types): (Vec<&Option<Ident>>, Vec<&Type>) = fields_named
        .named
        .iter()
        .map(|field| (&field.ident, &field.ty))
        .unzip();

    let view = quote! {
        Form::new()
            #(.with_input(
                Input::new(InputValue::Form(#idents.view()))
                    .with_hint(stringify!(#idents).to_case(Case::Title))
            ))*
            .into_iter()
            .flatten()
            .collect::<Form>()
            .with_title(stringify!(#ident).to_case(Case::Title))
            .readonly()
    };

    let enter = quote! {
        Form::new()
            #(.with_input(
                Input::new(InputValue::Form(#types::enter()))
                    .with_hint(stringify!(#idents).to_case(Case::Title))
            ))*
            .into_iter()
            .flatten()
            .collect::<Form>()
            .with_title(stringify!(#ident).to_case(Case::Title))
    };

    let update = quote! {
        Form::new()
            #(.with_input(
                Input::new(InputValue::Form(#idents.update()))
                    .with_hint(stringify!(#idents).to_case(Case::Title))
            ))*
            .into_iter()
            .flatten()
            .collect::<Form>()
            .with_title(stringify!(#ident).to_case(Case::Title))
    };

    [view, enter, update]
}

/// Generates form representations for a tuple struct or variant.
///
/// Assumes all fields of the struct are already in scope with identifiers `p0`, `p1`, ...
fn forms_tuple(ident: &Ident, fields_unnamed: &FieldsUnnamed) -> [TokenStream2; 3] {
    let (idents, types): (Vec<Ident>, Vec<&Type>) = fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(index, field)| (unnamed_ident(index, &field.ident), &field.ty))
        .unzip();

    let view = quote! {
        Form::new()
            #(.with_input(Input::new(InputValue::Form(#idents.view()))))*
            .into_iter()
            .flatten()
            .collect::<Form>()
            .with_title(stringify!(#ident).to_case(Case::Title))
            .readonly()
    };

    let enter = quote! {
        Form::new()
            #(.with_input(Input::new(InputValue::Form(#types::enter()))))*
            .into_iter()
            .flatten()
            .collect::<Form>()
            .with_title(stringify!(#ident).to_case(Case::Title))
    };

    let update = quote! {
        Form::new()
            #(.with_input(Input::new(InputValue::Form(#idents.update()))))*
            .into_iter()
            .flatten()
            .collect::<Form>()
            .with_title(stringify!(#ident).to_case(Case::Title))
    };

    [view, enter, update]
}

/// Generates form representations for a unit struct or variant.
fn forms_unit(ident: &Ident) -> [TokenStream2; 3] {
    let view = quote! {
        Form::new()
            .with_title(stringify!(#ident).to_case(Case::Title))
            .readonly()
    };

    let enter = quote! {
        Form::new()
            .with_title(stringify!(#ident)
            .to_case(Case::Title))
    };

    let update = quote! {
        Form::new()
            .with_title(stringify!(#ident)
            .to_case(Case::Title))
    };

    [view, enter, update]
}

/// Turns an index into an identifier `p0`, `p1`, ...
fn unnamed_ident(index: usize, ident: &Option<Ident>) -> Ident {
    Ident::new(format!("p{}", index).as_str(), ident.span())
}

/// Generates identifiers for fields in structs.
///
/// Structs:
///
/// ```
///  let name0 = &self.name0;
///  let name1 = &self.name1;
///  // ...
/// ```
///
/// Tuple structs:
///
/// ```
///  let p0 = &self.0;
///  let p1 = &self.1;
///  // ...
/// ```
///
/// Unit structs:
///
/// ```
/// // Generates nothing
/// ```
fn fields_idents_struct(fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|field| &field.ident)
            .map(|ident| quote! { let #ident = &self.#ident; })
            .collect(),
        Fields::Unnamed(fields_unnamed) => fields_unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| (Index::from(index), unnamed_ident(index, &field.ident)))
            .map(|(index, ident)| quote! { let #ident = &self.#index; })
            .collect(),
        Fields::Unit => quote! {},
    }
}

/// Generates identifiers for fields in enum variants.
///
/// Struct variants:
///
/// ```
/// { name0, name1, /* ... */ }
/// ```
///
/// Tuple variants:
///
/// ```
/// (p0, p1, /* ... */)
/// ```
///
/// Unit variants:
///
/// ```
/// // Generates nothing
/// ```
fn fields_idents_variant(fields: &Fields) -> TokenStream2 {
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
