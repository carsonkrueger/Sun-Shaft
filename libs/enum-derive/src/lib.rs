use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Data, DeriveInput, Ident,
};

// Structure to parse the target type from attribute
struct IntoTypeArgs {
    target_type: Ident,
}

// Implement parsing for the attribute arguments
impl Parse for IntoTypeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(IntoTypeArgs {
            target_type: input.parse::<Ident>()?,
        })
    }
}

#[proc_macro_derive(IntoEnum, attributes(into_enum))]
pub fn derive_into_enum(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Get the enum variant
    let data = match &input.data {
        Data::Enum(data) => data,
        _ => panic!("IntoEnum can only be derived for enums"),
    };

    // Extract the target type from attributes
    let target_enum = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("into_enum"))
        .map(|attr| attr.parse_args::<IntoTypeArgs>())
        .expect("Missing #[into_enum(TargetEnum)] attribute")
        .expect("Failed to parse into_enum attribute")
        .target_type;

    // Generate implementation
    let expanded = quote! {
        impl From<#name> for #target_enum {
            fn from(value: #name) -> Self {
                Self::#name(value)
            }
        }
    };

    TokenStream::from(expanded)
}
