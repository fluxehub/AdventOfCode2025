use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::Ident;
use syn::spanned::Spanned;
use syn::{ItemFn, parse_macro_input};

enum ReturnType {
    Result(proc_macro2::TokenStream),
    Plain(proc_macro2::TokenStream),
}

impl ReturnType {
    fn inner_type(&self) -> &proc_macro2::TokenStream {
        match self {
            ReturnType::Result(t) => t,
            ReturnType::Plain(t) => t,
        }
    }
}

enum ParseType {
    Line,
    Lines,
    Text,
}

// Adapted from https://github.com/gobanos/cargo-aoc/blob/v0.3/aoc-runner-derive/src/utils.rs
pub(crate) fn get_return_type(ty: &syn::Type) -> ReturnType {
    use syn::*;

    if let Type::Path(TypePath {
        path: Path { segments: s, .. },
        ..
    }) = ty
        && let Some(p) = s.last()
        && p.ident == "Result" // All string typing and no reflection makes Jack a dull boy
        && let PathArguments::AngleBracketed(a) = &p.arguments
        && let Some(GenericArgument::Type(t)) = a.args.first()
    {
        return crate::ReturnType::Result(quote! { #t });
    }

    crate::ReturnType::Plain(quote! { #ty })
}

fn get_parse_type(attr: TokenStream) -> Result<ParseType, syn::Error> {
    if attr.is_empty() {
        return Ok(ParseType::Text);
    }

    let arg: Ident = syn::parse(attr)?;

    match arg.to_string().as_str() {
        "text" => Ok(ParseType::Text),
        "line" => Ok(ParseType::Line),
        "lines" => Ok(ParseType::Lines),
        _ => Err(syn::Error::new(
            arg.span(),
            format!(
                "invalid parse type `{}`, expected `text`, `line`, or `lines`",
                arg
            ),
        )),
    }
}

fn create_parse_call(
    fn_name: &Ident,
    parse_type: &ParseType,
    return_type: &ReturnType,
) -> proc_macro2::TokenStream {
    match parse_type {
        ParseType::Line => match return_type {
            ReturnType::Plain(_) => {
                quote! { text.lines().map(|line: &str| #fn_name(line)).collect::<Vec<_>>() }
            }
            ReturnType::Result(_) => {
                quote! { text.lines().map(|line: &str| #fn_name(line)).collect::<Result<Vec<_>, _>>() }
            }
        },
        ParseType::Lines => quote! { #fn_name(text.lines()) },
        ParseType::Text => quote! { #fn_name(text) },
    }
}

#[proc_macro_attribute]
pub fn parse(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;
    let fn_block = &input.block;

    let parse_type = match get_parse_type(attr) {
        Ok(tt) => tt,
        Err(err) => return err.to_compile_error().into(),
    };

    let return_type = match &input.sig.output {
        syn::ReturnType::Default => {
            return syn::Error::new(input.span(), "Missing return type for parse")
                .to_compile_error()
                .into();
        }
        syn::ReturnType::Type(_, ty) => get_return_type(ty),
    };

    let parse_call = create_parse_call(fn_name, &parse_type, &return_type);

    let get_data = match return_type {
        ReturnType::Plain(_) => quote! { let data = #parse_call },
        ReturnType::Result(_) => {
            quote! { let data = #parse_call.expect("Unable to parse input") }
        }
    };

    let inner_return_type = return_type.inner_type();

    let parsed_data_type = match parse_type {
        ParseType::Line => quote! { Vec<#inner_return_type> },
        _ => quote! { #inner_return_type },
    };

    let expanded = quote! {
        static __PARSED_DATA: std::sync::OnceLock<#parsed_data_type> = std::sync::OnceLock::new();

        #fn_vis #fn_sig {
            #fn_block
        }

        fn __parse_data(text: &str) {
            #get_data;
            __PARSED_DATA.set(data).unwrap();
        }
    };

    TokenStream::from(expanded)
}

fn create_part_definition(part: u32, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;
    let fn_block = &input.block;

    let wrapper_name = match part {
        1 => Ident::new("__part_one", Span::call_site()),
        2 => Ident::new("__part_two", Span::call_site()),
        _ => panic!("Invalid part number"),
    };

    let part_literal = Literal::u32_unsuffixed(part);

    let expanded = quote! {
        #fn_vis #fn_sig {
            #fn_block
        }

        fn #wrapper_name() {
            let data = __PARSED_DATA.get().unwrap();
            let result = #fn_name(data);
            println!("Part {}: {}", #part_literal, result);
        }

        inventory::submit! {
            aoc::AocPart {
                part: #part_literal,
                func: #wrapper_name,
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn part_one(_attr: TokenStream, item: TokenStream) -> TokenStream {
    create_part_definition(1, item)
}

#[proc_macro_attribute]
pub fn part_two(_attr: TokenStream, item: TokenStream) -> TokenStream {
    create_part_definition(2, item)
}
