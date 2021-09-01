extern crate proc_macro;

use chitin_core::{ChitinEntry, FuncOrCode, Leaf, Request, ResponseTy};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use std::str::FromStr;
use syn::{parse_macro_input, Data, DeriveInput, Lit, Meta, NestedMeta, Type};

#[derive(Default)]
struct Args {
    named: HashMap<String, (String, usize)>,
    unnamed: Vec<String>,
}

impl Args {
    pub fn into_request_iter(self) -> impl Iterator<Item = Request> {
        let mut req: Vec<_> = self.named.into_iter().collect();
        req.sort_by(|(_, (_, pos1)), (_, (_, pos2))| pos1.cmp(pos2));
        req.into_iter().map(|(name, (ty, _))| Request { ty, name })
    }
}

enum EntryType {
    Leaf,
    Node,
    Uninit,
}

impl EntryType {
    fn from_str(s: &str) -> Self {
        match s {
            "request" => Self::Leaf,
            "router" => Self::Node,
            _ => panic!("{} 是啥？", s),
        }
    }
    fn gen_entry(
        &self,
        name: &str,
        key_value: &HashMap<String, String>,
        mut args: Args,
    ) -> ChitinEntry {
        match self {
            EntryType::Leaf => {
                let response = key_value.get("response").expect("找不到 response");
                ChitinEntry::Leaf {
                    name: name.to_owned(),
                    response_ty: ResponseTy(response.to_owned()),
                    request: args.into_request_iter().collect(),
                }
            }
            EntryType::Node => {
                let query_name = args.unnamed.pop().expect("router 項目必須單有一個參數");
                let query_ident = TokenStream2::from_str(&query_name).unwrap();
                ChitinEntry::Node {
                    name: name.to_owned(),
                    codegen: FuncOrCode::Code(quote! {
                        #query_ident::codegen_inner
                    }),
                    query_name,
                }
            }
            EntryType::Uninit => panic!("未指定項目的類型！"),
        }
    }
}

#[derive(Default)]
struct Fields {
    // (變數名, 類型)
    named: Vec<(String, String)>,
    // 只有類型
    unnamed: Vec<String>,
}

fn to_fields(syn_fields: &syn::Fields) -> Fields {
    let mut fields: Fields = Default::default();
    for field in syn_fields.iter() {
        if let Type::Path(p) = &field.ty {
            let ty = if let Some(ident) = p.path.get_ident() {
                ident.to_string()
            } else {
                p.path.to_token_stream().to_string().replace(" ", "")
            };
            if let Some(name) = field.ident.as_ref() {
                let name = name.to_string();
                fields.named.push((name, ty));
            } else {
                fields.unnamed.push(ty);
            }
        }
    }
    fields
}

struct PrimitiveRouter {
    pub name: String,
    pub next_enum: Ident,
}

#[proc_macro_derive(ChitinRouter, attributes(chitin))]
pub fn derive_router(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut leaves: Vec<chitin_core::Leaf> = Vec::new();
    let mut routers: Vec<PrimitiveRouter> = Vec::new();
    if let Data::Enum(data_enum) = ast.data {
        for variant in data_enum.variants.iter() {
            let variant_name = &variant.ident;

            // 解析 variant 有哪些 field
            let fields = to_fields(&variant.fields);

            for attr in variant.attrs.iter() {
                let list = if let Meta::List(list) = attr.parse_meta().unwrap() {
                    list
                } else {
                    continue;
                };

                let mut metas = list.nested.iter();
                let node_type = if let Some(first_meta) = metas.next() {
                    match first_meta {
                        NestedMeta::Meta(Meta::Path(p)) => p.get_ident().unwrap().to_string(),
                        _ => {
                            panic!("第一個參數需指名節點是 router 或 leaf");
                        }
                    }
                } else {
                    panic!("需指名節點是 router 或 leaf")
                };

                // 此 variant 是一個路由
                if node_type == "router" {
                    assert!(metas.next() == None, "router 類型無需額外元訊息");
                    // TODO: 去找 rust 正確的術語以給出更恰當的錯誤訊息
                    assert!(fields.unnamed.len() == 1, "router 類型僅需一個無名參數");
                    assert!(fields.named.len() == 0, "router 類型僅需一個無名參數");
                    routers.push(PrimitiveRouter {
                        name: variant_name.to_string(),
                        next_enum: Ident::new(&fields.unnamed[0], Span::call_site()),
                    });
                // 此 variant 是一個葉子
                } else if node_type == "leaf" {
                    if let Some(second_meta) = metas.next() {
                        let response_ty = match second_meta {
                            NestedMeta::Meta(Meta::NameValue(p)) => {
                                if let Lit::Str(lit) = &p.lit {
                                    let key = p.path.get_ident().unwrap().to_string();
                                    if key == "response" {
                                        let value = lit.value().to_string();
                                        value
                                    } else {
                                        panic!("leaf 類型元訊息的鍵只能是 response");
                                    }
                                } else {
                                    panic!("response 的值該是一個 literal");
                                }
                            }
                            _ => {
                                panic!("")
                            }
                        };
                        let args: Vec<chitin_core::Argument> = fields
                            .named
                            .iter()
                            .map(|(name, ty)| chitin_core::Argument {
                                name: name.to_string(),
                                ty: ty.to_string(),
                            })
                            .collect();
                        leaves.push(Leaf {
                            name: variant_name.to_string(),
                            response_ty,
                            args,
                        })
                    }
                    assert!(metas.next() == None, "leaf 類型僅需一個鍵值對");
                } else {
                    panic!(
                        "未知的節點類型 {} ，節點類型只能是 router 或 leaf",
                        node_type
                    )
                }
            }
        }
    } else {
        panic!("只有枚舉類型可以實作 ChitinCodegen")
    };

    let names: Vec<&String> = routers.iter().map(|router| &router.name).collect();
    let next_enums: Vec<&Ident> = routers.iter().map(|router| &router.next_enum).collect();

    let ident = ast.ident; // 枚舉名
    let expanded = quote! {
        #[automatically_derived]
        impl #ident {
            pub fn get_children() -> Vec<chitin::ChitinEntry2> {
                let leaves = vec![#(#leaves),*];
                let mut children: Vec<ChitinEntry2> = leaves
                    .into_iter()
                    .map(|leaf| ChitinEntry2::Leaf(leaf))
                    .collect();
                let mut routers: Vec<ChitinEntry2> = vec![#( chitin::Router{ name: #names.to_owned(), children: #next_enums::get_children() }.into()  ),*];
                // for router in routers.iter() {
                //     let next_enum = router.next_enum;
                //     children.push(ChitinEntry2::Router{
                //         name: router.name,
                //         children: (#next_enum).get_children()
                //     })
                // }
                children.append(&mut routers);
                children
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(ChitinCodegen, attributes(chitin))]
pub fn derive_router2(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut entries = Vec::<ChitinEntry>::new();
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
                            entry_type = EntryType::from_str(&p.get_ident().unwrap().to_string());
                        }
                        _ => panic!(),
                    }
                }
            }
            for (pos, field) in variant.fields.iter().enumerate() {
                if let Type::Path(p) = &field.ty {
                    let ty = if let Some(ident) = p.path.get_ident() {
                        ident.to_string()
                    } else {
                        p.path.to_token_stream().to_string().replace(" ", "")
                    };
                    if let Some(name) = field.ident.as_ref() {
                        let name = name.to_string();
                        args.named.insert(name, (ty, pos));
                    } else {
                        args.unnamed.push(ty);
                    }
                }
            }
            let entry = entry_type.gen_entry(&entry_name, &map, args);
            entries.push(entry);
        }
    } else {
        panic!("只有枚舉類型可以實作 ChitinRouter")
    };

    let entries = entries.iter();
    let ident = ast.ident;
    let name = ident.to_string();
    let expanded = quote! {
        #[automatically_derived]
        impl ChitinCodegen for #ident {
            fn get_name() -> &'static str {
                #name
            }
            fn get_entries() -> Vec<ChitinEntry> {
                vec![#(#entries),*]
            }
        }
    };
    TokenStream::from(expanded)
}
