use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub mod chitin_util;

#[derive(Clone, Debug)]
pub enum CodegenOption {
    Server {
        error: &'static str,
        context: &'static str,
    },
    Client {
        error: &'static str,
    },
}
impl CodegenOption {
    pub fn error_type(&self) -> &'static str {
        match self {
            Self::Server { error, .. } => error,
            Self::Client { error } => error,
        }
    }
    pub fn ctx_type(&self) -> Option<&'static str> {
        match self {
            Self::Server { context, .. } => Some(context),
            _ => None,
        }
    }
    pub fn is_server(&self) -> bool {
        match self {
            Self::Server { .. } => true,
            _ => false,
        }
    }
}

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

fn gen_arg_string(requests: &[Request], with_type: bool, opt: &CodegenOption) -> String {
    requests
        .iter()
        .map(|req| {
            if with_type {
                let ty = if opt.is_server() {
                    req.ty.clone()
                } else {
                    chitin_util::to_typescript_type(&req.ty)
                };
                format!("{}: {}", req.name, ty)
            } else {
                req.name.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub enum FuncOrCode {
    Func(fn(&CodegenOption, &mut Vec<String>) -> String),
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
impl FuncOrCode {
    fn gen_string(&self, opt: &CodegenOption, prev: &mut Vec<String>) -> String {
        match self {
            FuncOrCode::Func(f) => f(opt, prev),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct ResponseTy(pub String);
impl ResponseTy {
    pub fn as_result(&self, opt: &CodegenOption) -> String {
        format!("Result<{}, {}>", self.0, opt.error_type())
    }
}
#[derive(Debug)]
pub enum ChitinEntry {
    Leaf {
        name: String,
        response_ty: ResponseTy,
        request: Vec<Request>,
    },
    Node {
        name: String,
        query_name: String,
        codegen: FuncOrCode,
    },
}

impl ToTokens for ChitinEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ChitinEntry::Node {
                name,
                query_name,
                codegen,
            } => {
                if let FuncOrCode::Code(code) = codegen {
                    tokens.extend(quote! {
                        ChitinEntry::Node {
                            name: #name.to_owned(),
                            query_name: #query_name.to_owned(),
                            codegen: FuncOrCode::Func(#code)
                        }
                    });
                } else {
                    panic!("內部實作錯誤")
                }
            }
            ChitinEntry::Leaf {
                name,
                response_ty,
                request,
            } => {
                let request = request.iter();
                let response_ty = &response_ty.0;
                tokens.extend(quote! {
                    ChitinEntry::Leaf {
                        name: #name.to_owned(),
                        response_ty: ResponseTy(#response_ty.to_owned()),
                        request: vec![#(#request),*]
                    }
                });
            }
        }
    }
}
fn client_codegen_inner(
    opt: &CodegenOption,
    entries: &[ChitinEntry],
    prev: &mut Vec<String>,
) -> String {
    let mut code = "".to_owned();
    for entry in entries.iter() {
        match entry {
            ChitinEntry::Leaf {
                name,
                response_ty,
                request,
            } => {
                code.push_str(&format!(
                    "    async {}({}): Promise<{}> {{\n",
                    get_query_func_name(name),
                    gen_arg_string(request, true, opt),
                    chitin_util::to_typescript_type(&response_ty.as_result(&opt))
                ));
                prev.push(name.clone());
                code.push_str(&format!(
                    "        return JSON.parse(await this.fetchResult({}));\n",
                    chitin_util::gen_enum_json(prev, request)
                ));
                prev.pop();
                code.push_str("    }\n");
            }
            ChitinEntry::Node { name, codegen, .. } => {
                prev.push(name.clone());
                code.push_str(&codegen.gen_string(&opt, prev));
                prev.pop();
            }
        }
    }
    code
}

pub trait ChitinCodegen {
    fn get_name() -> &'static str;
    fn get_entries() -> Vec<ChitinEntry>;
    fn codegen(opt: &CodegenOption) -> String {
        if opt.is_server() {
            Self::server_codegen(opt)
        } else {
            let mut code = "".to_owned();
            code.push_str(&format!(
                "export abstract class {}Fetcher {{\n",
                Self::get_name()
            ));
            code.push_str("    abstract fetchResult(query: Object): Promise<string>;\n");
            code.push_str(&Self::codegen_inner(opt, &mut vec![]));
            code.push_str("}\n");
            code
        }
    }
    fn codegen_inner(opt: &CodegenOption, prev: &mut Vec<String>) -> String {
        if opt.is_server() {
            Self::server_codegen(opt)
        } else {
            client_codegen_inner(opt, &Self::get_entries(), prev)
        }
    }
    fn server_codegen(opt: &CodegenOption) -> String {
        let entries = Self::get_entries();
        let mut routers_name = vec![];
        let mut code = "".to_owned();
        for entry in entries.iter() {
            if let ChitinEntry::Node {
                query_name,
                codegen,
                ..
            } = entry
            {
                routers_name.push(get_router_name(query_name));
                code.push_str(&codegen.gen_string(opt, &mut vec![]));
            }
        }

        code.push_str(&format!(
            "#[async_trait]\npub trait {} {{\n",
            get_router_name(&Self::get_name())
        ));
        for router_name in routers_name.iter() {
            code.push_str(&format!(
                "    type {}: {} + Sync;\n",
                router_name, router_name
            ));
        }
        // NOTE: 錯誤處理
        code.push_str(&format!(
            "    fn on_error(&self, _err: &{}) {{}}\n",
            opt.error_type()
        ));

        for entry in entries.iter() {
            match entry {
                ChitinEntry::Leaf {
                    name,
                    response_ty,
                    request,
                } => {
                    code.push_str(&format!(
                        "    async fn {}(&self, context: {}, {}) -> {};\n",
                        get_handler_name(name, false),
                        opt.ctx_type().unwrap(),
                        gen_arg_string(request, true, opt),
                        &response_ty.as_result(&opt)
                    ));
                }
                ChitinEntry::Node {
                    name, query_name, ..
                } => {
                    code.push_str(&format!(
                        "    fn {}(&self) -> &Self::{};\n",
                        get_handler_name(name, true),
                        get_router_name(query_name)
                    ));
                }
            }
        }
        code.push_str(&format!(
            "    async fn handle(&self, context: {}, query: {}) -> Result<String, Error> {{\n",
            opt.ctx_type().unwrap(),
            Self::get_name()
        ));
        code.push_str("        match query {\n");
        for entry in entries.iter() {
            match entry {
                ChitinEntry::Leaf { name, request, .. } => {
                    code.push_str(&format!(
                        "             {}::{} {{ {} }} => {{\n",
                        Self::get_name(),
                        name,
                        gen_arg_string(request, false, opt)
                    ));
                    code.push_str(&format!(
                        "                 let resp = self.{}(context, {}).await;\n",
                        get_handler_name(name, false),
                        gen_arg_string(request, false, opt)
                    ));
                    code.push_str(&format!(
                        "                 if let Err(err) = &resp {{\n self.on_error(err);\n }}\n",
                    ));
                    code.push_str("                 serde_json::to_string(&resp)\n");
                }
                ChitinEntry::Node { name, .. } => {
                    code.push_str(&format!(
                        "             {}::{}(query) => {{\n",
                        Self::get_name(),
                        name
                    ));
                    code.push_str(&format!(
                        "                 self.{}().handle(context, query).await\n",
                        get_handler_name(name, true)
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

pub fn get_handler_name(name: &str, is_router: bool) -> String {
    if is_router {
        format!("{}_router", name.to_snake_case())
    } else {
        name.to_snake_case()
    }
}

pub fn get_query_func_name(query_name: &str) -> String {
    query_name.to_camel_case()
}
