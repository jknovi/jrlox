use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, Token, Type};

// TODO: Add the union types, need to figure out how to define a type that can have variants:

struct Grammar {
    rules: Punctuated<Rule, Token![;]>,
}

impl Parse for Grammar {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            rules: Punctuated::<Rule, Token![;]>::parse_terminated(input)?,
        })
    }
}

struct Rule {
    name: Ident,
    fields: Punctuated<Field, Token![,]>,
}

struct Field {
    name: Ident,
    ty: Type,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;

        Ok(Field { name, ty })
    }
}

impl Parse for Rule {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let fields = Punctuated::<Field, Token![,]>::parse_separated_nonempty(input)?;

        Ok(Self { name, fields })
    }
}

#[proc_macro]
pub fn grammar(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Grammar { rules } = parse_macro_input!(input as Grammar);

    let rendered_structs = rules.iter().map(render_rule_struct);

    let expanded = quote! {
        #(#rendered_structs)*
    };

    proc_macro::TokenStream::from(expanded)
}

fn render_rule_struct(Rule { name, fields }: &Rule) -> TokenStream {
    let rendered_fields = fields
        .iter()
        .map(|Field { name, ty }| quote! {#name: ::std::boxed::Box<#ty>});

    quote! {
        pub struct #name {
            #(#rendered_fields),*
        }
    }
}
