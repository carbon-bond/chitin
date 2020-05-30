extern crate proc_macro;

use chitin_core::{Entry, Request};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Lit, Meta, NestedMeta, Type};
use std::collections::HashMap;

#[derive(Default)]
struct Args {
    named: HashMap<String, String>,
    unnamed: Vec<String>
}

enum EntryType {
    Leaf,
    Node,
    Uninit,
}

impl EntryType {
    fn from_str(name: &str, s: &str) -> Self {
        match s {
            "request" => Self::Leaf,
            "router" => Self::Node,
            _ => panic!(format!("{} 是啥？", s)),
        }
    }
    fn gen_entry(&self, name: &str, key_value: &HashMap<String, String>, mut args: Args) -> Entry {
        match self {
            EntryType::Leaf => {
                let response = key_value.get("response").expect("找不到 response");
                Entry::Leaf {
                    name: name.to_owned(),
                    response_ty: response.to_owned(),
                    request: args.named.into_iter().map(|(name, ty)| {
                        Request { name, ty }
                    }).collect()
                }
            },
            EntryType::Node => {
                Entry::Node {
                    name: name.to_owned(),
                    query_name: args.unnamed.pop().expect("router 項目必須單有一個參數")
                }
            },
            _ => panic!("未指定項目的類型！")
        }
    }
}

#[proc_macro_derive(ChitinRouter, attributes(chitin))]
pub fn derive_router(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;
    let router_name = format!("{}Router", ident);
    let mut entries = Vec::<Entry>::new();
    if let Data::Enum(data_enum) = ast.data {
        for variant in data_enum.variants.iter() {
            let entry_name = variant.ident.to_string();
            let mut entry_type = EntryType::Uninit;
            let mut map = HashMap::<String, String>::new();
            let mut args = Args::default();
            for attr in variant.attrs.iter() {
                let list = if let Meta::List(list) = attr.parse_meta().unwrap() {
                    list
                } else {
                    continue;
                };

                for meta in list.nested.iter() {
                    match meta {
                        NestedMeta::Meta(Meta::NameValue(p)) => {
                            if let Lit::Str(lit) = &p.lit {
                                let key = p.path.get_ident().unwrap().to_string();
                                let value = lit.value().to_string();
                                map.insert(key, value);
                            } else {
                                panic!();
                            }
                        }
                        NestedMeta::Meta(Meta::Path(p)) => {
                            entry_type = EntryType::from_str(
                                &entry_name,
                                &p.get_ident().unwrap().to_string(),
                            );
                        }
                        _ => panic!(),
                    }
                }
            }
            for field in variant.fields.iter() {
                if let Type::Path(p) = &field.ty {
                    let ty = p.path.get_ident().unwrap().to_string();
                    if let Some(name) = field.ident.as_ref() {
                        let name = name.to_string();
                        args.named.insert(name, ty);
                    } else {
                        args.unnamed.push(ty);
                    }
                }
            }
            let entry = entry_type.gen_entry(&entry_name, &map, args);
            // }
            // if let Some(Entry::Node {
            //     ref mut enum_name, ..
            // }) = entry
            // {
            //     for field in variant.fields.iter() {
            //         if enum_name.is_some() {
            //             panic!("router 項目只能有一個參數（即 query 枚舉）");
            //         }
            //         if let Type::Path(p) = &field.ty {
            //             let ty = p.path.get_ident().unwrap().to_string();
            //             *enum_name = Some(ty);
            //         }
            //     }
            // }
            entries.push(entry);
        }
    } else {
        panic!("只有枚舉類型可以實作 ChitinRouter")
    };
    let entries = entries.iter();
    let expanded = quote! {
        #[automatically_derived]
        impl ChitinRouter for #ident {
            fn get_router_name() -> &'static str {
                #router_name
            }
            fn get_entries() -> Vec<Entry> {
                vec![#(#entries),*]
            }
        }
    };
    TokenStream::from(expanded)
}
