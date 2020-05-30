extern crate proc_macro;

use chitin_core::{Entry, Request};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Lit, Meta, NestedMeta, Type};

enum EntryType {
    Leaf(String),
    Node(String),
    Uninit,
}

impl EntryType {
    fn from_str(name: &str, s: &str) -> Self {
        match s {
            "request" => Self::Leaf(name.to_owned()),
            "router" => Self::Node(name.to_owned()),
            _ => panic!(format!("{} 是啥？", s)),
        }
    }
    fn gen_entry(&self, key: &str, value: &str) -> Entry {
        match self {
            EntryType::Leaf(name) => {
                if key == "response" {
                    Entry::Leaf {
                        name: name.to_owned(),
                        response_ty: value.to_owned(),
                        request: vec![],
                    }
                } else {
                    panic!("未知的參數：{}", key)
                }
            }
            EntryType::Node(name) => unimplemented!(),
            _ => panic!("未指定項目的類型！"),
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
            let mut entry: Option<Entry> = None;
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
                                entry = Some(entry_type.gen_entry(&key, &value));
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
            if let Some(Entry::Leaf {
                ref mut request, ..
            }) = entry
            {
                for field in variant.fields.iter() {
                    if let Type::Path(p) = &field.ty {
                        let name = field.ident.as_ref().unwrap().to_string();
                        let ty = p.path.get_ident().unwrap().to_string();
                        request.push(Request { ty, name });
                    }
                }
            }
            entries.push(entry.unwrap());
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
