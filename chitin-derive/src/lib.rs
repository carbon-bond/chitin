extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Meta, NestedMeta,Lit};
use chitin_core::Entry;

#[proc_macro_derive(ChitinRouter, attributes(chitin))]
pub fn derive_router(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;
    let server_name = format!("{}Server", ident);
    // let mut entries = Vec::<Entry>::new();
    if let Data::Enum(data_enum) = ast.data {
        for variant in data_enum.variants.iter() {
            for attr in variant.attrs.iter() {
                if let Meta::List(list) = attr.parse_meta().unwrap() {
                    for meta in list.nested.iter() {
                        if let NestedMeta::Meta(meta) = meta {
                                match meta {
                                    Meta::NameValue(p) => {
                                        p.path.get_ident().unwrap().to_string();
                                        if let Lit::Str(lit) = &p.lit {
                                            println!("bbb {}", lit.value());
                                        } else {
                                            panic!();
                                        }
                                    }
                                    Meta::Path(p) => {
                                        println!("hhh {}", p.get_ident().unwrap().to_string());
                                    }
                                    _ => panic!(),
                                }
                        } else {
                            panic!();
                        }
                    }
                }
            }
        }
    } else {
        panic!("只有枚舉類型可以實作 ChitinRouter")
    };
    let expanded = quote! {
        #[automatically_derived]
        impl ChitinRouter for #ident {
            fn get_entries() -> Vec<Entry> {
                vec![]
            }
        }
    };
    TokenStream::from(expanded)
}
