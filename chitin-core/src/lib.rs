use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub ty: String,
    pub name: String,
}
impl ToTokens for Request {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = &self.ty;
        let name = &self.name;
        tokens.extend(quote! {
            Request {
                ty: #ty.to_owned(),
                name: #name.to_owned(),
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entry {
    Leaf {
        name: String,
        response_ty: String,
        request: Vec<Request>,
    },
    Node {
        name: String,
        query_name: String,
    },
}

impl ToTokens for Entry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Entry::Node { name, query_name } => {
                tokens.extend(quote! {
                    Entry::Node {
                        name: #name.to_owned(),
                        query_name: #query_name.to_owned(),
                    }
                });
            }
            Entry::Leaf {
                name,
                response_ty,
                request,
            } => {
                let request = request.iter();
                tokens.extend(quote! {
                    Entry::Leaf {
                        name: #name.to_owned(),
                        response_ty: #response_ty.to_owned(),
                        request: vec![#(#request),*]
                    }
                });
            }
        }
    }
}

pub trait ChitinCodegen {
    fn get_router_name() -> &'static str;
    fn get_entries() -> Vec<Entry>;
    fn gen_server_code() -> String {
        unimplemented!();
    }
}
