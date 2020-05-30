use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct CodegenOption {}

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

fn gen_arg_string(requests: &[Request]) -> String {
    if requests.len() == 0 {
        "".to_owned()
    } else {
        let mut args = format!("{}: {}", requests[0].name, requests[0].ty);
        for req in &requests[1..] {
            args.push_str(&format!(", {}: {}", req.name, req.ty));
        }
        args
    }
}

pub enum FuncOrCode {
    Func(fn(&CodegenOption) -> String),
    Code(TokenStream),
}
impl std::fmt::Debug for FuncOrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FuncOrCode::Code(code) => {
                write!(f, "Code({})", code.to_string())?;
            }
            FuncOrCode::Func(_) => {
                write!(f, "Function")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Entry {
    Leaf {
        name: String,
        response_ty: String,
        request: Vec<Request>,
    },
    Node {
        name: String,
        query_name: String,
        codegen: FuncOrCode,
    },
}

impl ToTokens for Entry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Entry::Node {
                name,
                query_name,
                codegen,
            } => {
                if let FuncOrCode::Code(code) = codegen {
                    tokens.extend(quote! {
                        Entry::Node {
                            name: #name.to_owned(),
                            query_name: #query_name.to_owned(),
                            codegen: FuncOrCode::Func(#code)
                        }
                    });
                } else {
                    panic!("內部實作錯誤")
                }
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
    fn get_name() -> &'static str;
    fn get_entries() -> Vec<Entry>;
    fn codegen(opt: &CodegenOption) -> String {
        let entries = Self::get_entries();
        let mut routers_name = vec![];
        let mut code = "".to_owned();
        for entry in entries.iter() {
            if let Entry::Node {
                query_name,
                codegen,
                ..
            } = entry
            {
                routers_name.push(get_router_name(query_name));
                if let FuncOrCode::Func(f) = codegen {
                    code.push_str(&f(opt));
                }
            }
        }

        code.push_str(&format!(
            "#[async trait]\ntrait {} {{\n",
            get_router_name(&Self::get_name())
        ));
        for router_name in routers_name.iter() {
            code.push_str(&format!("    type {};\n", router_name));
        }
        for entry in entries.iter() {
            match entry {
                Entry::Leaf {
                    name,
                    response_ty,
                    request,
                } => {
                    code.push_str(&format!(
                        "    async fn {}(&self, {}) -> {};\n",
                        get_handler_name(name),
                        gen_arg_string(request),
                        response_ty
                    ));
                }
                Entry::Node {
                    name, query_name, ..
                } => {
                    code.push_str(&format!(
                        "    fn {}(&self, query: {}) -> Self::{};\n",
                        get_handler_name(name),
                        query_name,
                        get_router_name(query_name)
                    ));
                }
            }
        }
        code.push_str(&format!(
            "    async fn handle(&self, query: {}) -> Result<String, Error> {{\n",
            Self::get_name()
        ));
        code.push_str("        match query {\n");
        for entry in entries.iter() {
            match entry {
                Entry::Leaf { name, request, .. } => {
                    code.push_str(&format!(
                        "             {}::{} {{ {} }} => {{\n",
                        Self::get_name(),
                        name,
                        gen_arg_string(request)
                    ));
                    code.push_str(&format!(
                        "                 let resp = self.{}().await;\n",
                        get_handler_name(name)
                    ));
                    code.push_str(&format!("                 serde_json::to_string(&resp)\n",));
                }
                Entry::Node {
                    name, query_name, ..
                } => {
                    code.push_str(&format!(
                        "             {}::{}(query) => {{\n",
                        Self::get_name(),
                        name
                    ));
                    code.push_str(&format!(
                        "                 self.{}().handle(query).await\n",
                        get_handler_name(name)
                    ));
                }
            }
            code.push_str("             }\n");
        }
        code.push_str("        }\n");
        code.push_str("    }\n");
        code.push_str("}\n");
        code
    }
}

pub fn get_router_name(query_name: &str) -> String {
    format!("{}Router", query_name)
}

pub fn get_handler_name(name: &str) -> String {
    format!("{}", name)
}
