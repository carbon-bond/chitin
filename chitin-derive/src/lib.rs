extern crate proc_macro;

use chitin_core::Leaf;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Lit, Meta, NestedMeta, Type};

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
    let enum_name = ident.clone().to_string(); // 枚舉名
    let expanded = quote! {
        #[automatically_derived]
        impl #ident {
            pub fn get_entry(variant_name: Option<String>) -> chitin::ChitinEntry {
                let leaves = vec![#(#leaves),*];
                let routers = vec![#( #next_enums::get_entry(Some(#names.to_owned())) ),*];
                chitin::ChitinEntry {
                    name: #enum_name.to_owned(),
                    variant_name,
                    routers,
                    leaves,
                }
            }
            pub fn get_root_entry() -> chitin::ChitinEntry {
                Self::get_entry(None)
            }
        }
    };
    TokenStream::from(expanded)
}
