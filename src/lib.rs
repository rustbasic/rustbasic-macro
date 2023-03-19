//! # Rust Basic Macro
//!
//! `RustBasic` is a planned development that aims to make Rust easy to learn, teach, and use.

// rustbasic macro - lib.rs
// Thanks to wusyong & team ( https://github.com/wusyong/smol-potat ) for the reference.
// Thanks to Fishrock123 & team ( https://github.com/async-rs/async-attributes ) for the reference.
#![allow(unused_doc_comments)]
#![allow(dead_code)]

#![allow(unused_imports)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Expr;

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let opts = syn::parse_macro_input!(attr as Opts);

    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    #[allow(unused_variables)]
    let attrs = &input.attrs;
    let vis = &input.vis;

    let crate_root = opts.crate_root;

    let threads = match opts.threads {
        Some((num, span)) => {
            let num = num.to_string();
            Some(quote_spanned!(span=> #num))
        }
        #[cfg(feature = "auto")]
        None => Some(quote! {
            #crate_root::std::string::ToString::to_string(
                &#crate_root::std::cmp::max(#crate_root::num_cpus::get(), 1)
            )
        }),
        #[cfg(not(feature = "auto"))]
        None => None,
    };

    #[allow(unused_variables)]
    let set_threads = threads.map(|threads| {
        quote! {
            #crate_root::std::env::set_var(
                "RUSTBASIC-THREADS",
                #threads,
            );
        }
    });

    let result : proc_macro2::TokenStream;

    match input.sig.asyncness {
        None | Some(_) if input.sig.ident != "main" => {    // => If it is not async or not main(),
            result = quote! {
                #vis fn #name(#inputs) #ret {
                    println!("Hello, macro attribute.");
                    #body
                }
            }
        }
        _ => {                                              //  => If it is async main().
            result = quote! {
                #vis fn main() #ret {
                    println!("Hello, macro attribute.");
                    futures::executor::block_on(async_main());
                }
                async fn async_main() #ret {
                    #body
                }
            }
        }
    }

    result.into()
}

struct Opts {
    crate_root: syn::Path,
    threads: Option<(u32, Span)>,
}

impl Parse for Opts {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut crate_root = None;
        let mut threads = None;

        loop {
            if input.is_empty() {
                break;
            }

            let name_value: syn::MetaNameValue = input.parse()?;
            let ident = match name_value.path.get_ident() {
                Some(ident) => ident,
                None => {
                    return Err(syn::Error::new_spanned(
                        name_value.path,
                        "Must be a single ident",
                    ))
                }
            };
            match &*ident.to_string().to_lowercase() {
                "threads" => match &name_value.lit {
                    syn::Lit::Int(expr) => {
                        if threads.is_some() {
                            return Err(syn::Error::new_spanned(
                                name_value,
                                "multiple threads argments",
                            ));
                        }

                        let num = expr.base10_parse::<std::num::NonZeroU32>()?;
                        threads = Some((num.get(), expr.span()));
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            name_value,
                            "threads argument must be an integer",
                        ))
                    }
                },
                "crate" => match &name_value.lit {
                    syn::Lit::Str(path) => {
                        if crate_root.is_some() {
                            return Err(syn::Error::new_spanned(
                                name_value,
                                "multiple crate arguments",
                            ));
                        }

                        crate_root = Some(path.parse()?);
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            name_value,
                            "crate argument must be a string",
                        ))
                    }
                },
                name => {
                    return Err(syn::Error::new_spanned(
                        name,
                        "unknown attribute {}, expected `threads` or `crate`",
                    ));
                }
            }

            input.parse::<Option<syn::Token![,]>>()?;
        }

        Ok(Self {
            crate_root: crate_root.unwrap_or_else(|| syn::parse2(quote!(::rustbasic)).unwrap()),
            threads,
        })
    }
}
