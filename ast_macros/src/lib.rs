use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, Token, Type};

// TODO:
//   - Only box those types that are recursive
//   - Error handling
//   - docs?

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
    body: RuleBody,
}

impl Parse for Rule {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;

        input.parse::<Token![=>]>()?;

        let body = input.parse()?;

        Ok(Self { name, body })
    }
}

enum RuleBody {
    Unique {
        fields: Punctuated<Field, Token![,]>,
    },
    Union {
        variants: Punctuated<UnionVariant, Token![|]>,
    },
}

impl Parse for RuleBody {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) && input.peek2(Token![:]) {
            // Unique expresions start with a field that is: ``ident: Value``
            let fields = Punctuated::<Field, Token![,]>::parse_separated_nonempty(input)?;

            Ok(Self::Unique { fields })
        } else {
            let variants = Punctuated::<UnionVariant, Token![|]>::parse_separated_nonempty(input)?;

            Ok(Self::Union { variants })
        }
    }
}

enum UnionVariant {
    /// This variant name and type are the same.
    ImplicitType(Type),

    /// This variant doesn't really have a type, can be a reserved word, for example.
    /// To distinguish from ImplicitTypes and Atoms, a '@' needs to be prefixed to the atom.
    Atom(Ident),

    /// The name of the variant and the type are different, usually for variants whose types
    /// are primitive or 3p types. It can be desirable to add an alias that makes intent more
    /// explicit or when 2 different productions of the rule may have the same underlying type, but
    /// semantically are different.
    ///
    /// To Alias a type defined like: `AliasName as UnderlyingType`.
    Aliased { alias: Ident, ty: Type },
}

impl Parse for UnionVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![@]) {
            // Atoms start with @
            input.parse::<Token![@]>()?;

            let ident = input.parse()?;

            Ok(Self::Atom(ident))
        } else if input.peek(Ident) && input.peek2(Token![as]) {
            // as is used to alias stuff

            let alias = input.parse()?;

            input.parse::<Token![as]>()?;

            let ty = input.parse()?;

            Ok(Self::Aliased { alias, ty })
        } else {
            let ty = input.parse()?;

            Ok(Self::ImplicitType(ty))
        }
    }
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

#[proc_macro]
pub fn grammar(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Grammar { rules } = parse_macro_input!(input as Grammar);

    let rendered_structs = rules.iter().map(render_rule_struct);

    let names = rules.iter().map(|Rule { name, .. }| name).collect();
    let visitor_definition = define_visitors(names);

    let expanded = quote! {
        #(#rendered_structs)*

        #visitor_definition
    };

    proc_macro::TokenStream::from(expanded)
}

fn render_rule_struct(Rule { name, body }: &Rule) -> TokenStream {
    match body {
        RuleBody::Unique { fields } => {
            let rendered_fields = fields
                .iter()
                .map(|Field { name, ty }| quote! {#name: ::std::boxed::Box<#ty>});

            quote! {
                pub struct #name {
                    #(pub #rendered_fields),*
                }
            }
        }
        RuleBody::Union { variants: branches } => {
            let rendered_variants = branches.iter().map(render_branched_variant);

            quote! {
                pub enum #name {
                    #(#rendered_variants),*
                }
            }
        }
    }
}

fn render_branched_variant(variant: &UnionVariant) -> TokenStream {
    match variant {
        UnionVariant::ImplicitType(ty) => quote! { #ty(#ty) },
        UnionVariant::Atom(atom) => quote! { #atom },
        UnionVariant::Aliased { alias, ty } => quote! { #alias(#ty) },
    }
}

fn define_visitors(names: Vec<&Ident>) -> TokenStream {
    let func_names = names
        .iter()
        .map(|name| {
            Ident::new(
                &format!("visit_{}", name.to_string().to_case(Case::Snake)),
                name.span(),
            )
        })
        .collect::<Vec<_>>();

    quote! {
        pub trait SyntaxVisitor<T> {
            #(fn #func_names(&mut self, arg: &#names) -> T;)*
        }

        pub trait Visitable<T> {
            fn accept(&self, visitor: &mut impl SyntaxVisitor<T>) -> T;
        }

        #(impl<T> Visitable<T> for #names {
            fn accept(&self, visitor: &mut impl SyntaxVisitor<T>) -> T {
                visitor.#func_names(&self)
            }
        })*
    }
}
