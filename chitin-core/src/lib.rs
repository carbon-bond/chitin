use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub mod chitin_util;

#[derive(Debug)]
pub struct Argument {
    pub name: String,
    pub ty: String,
}

impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let ty = &self.ty;
        tokens.extend(quote! {
            {
                use chitin::Argument;
                Argument {
                    ty: #ty.to_owned(),
                    name: #name.to_owned(),
                }
            }
        });
    }
}

#[derive(Debug)]
pub struct Leaf {
    pub name: String,
    pub response_ty: String,
    pub args: Vec<Argument>,
}

impl ToTokens for Leaf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let response_ty = &self.response_ty;
        let args = &self.args;
        tokens.extend(quote! {
            {
                use chitin::Leaf;
                use chitin::Argument;
                Leaf {
                    response_ty: #response_ty.to_owned(),
                    name: #name.to_owned(),
                    args: vec![#(#args),*]
                }
            }
        });
    }
}

#[derive(Debug)]
pub struct ChitinEntry {
    pub name: String,
    pub variant_name: Option<String>,
    pub leaves: Vec<Leaf>,
    pub routers: Vec<ChitinEntry>,
}

pub enum Language {
    Rust,
    TypeScript,
}

pub enum Side {
    Server { context: &'static str },
    Client,
}

pub struct CodegenOption {
    pub side: Side,
    pub language: Language,
    pub error: &'static str,
}

impl CodegenOption {
    pub fn prelude(&self) -> String {
        match self {
            CodegenOption {
                side: Side::Client,
                language: Language::TypeScript,
                ..
            } => {
                let mut code = format!("export type Option<T> = T | null;\n");
                code.push_str(
                    "export type Result<T, E> = {
    'Ok': T
} | {
    'Err': E
};\n",
                );
                code.push_str("export type Fetcher = (query: Object) => Promise<string>;\n");
                code
            }
            CodegenOption {
                side: Side::Server { .. },
                language: Language::Rust,
                ..
            } => {
                let mut code = format!("use async_trait::async_trait;\n");
                code.push_str("use serde_json::error::Error;\n");
                code
            }
            _ => {
                unimplemented!()
            }
        }
    }
    pub fn gen_response_ty(&self, response_ty: &String) -> String {
        match self {
            CodegenOption {
                side: Side::Client,
                language: Language::TypeScript,
                error,
            } => {
                let primitive = format!("Result<{}, {}>", response_ty, error);
                let ts_type = chitin_util::to_typescript_type(&primitive);
                ts_type
            }
            CodegenOption {
                side: Side::Server { .. },
                language: Language::Rust,
                error,
            } => {
                format!("Result<{}, {}>", response_ty, error)
            }
            _ => {
                unimplemented!()
            }
        }
    }
    pub fn gen_args(&self, args: &Vec<Argument>, has_type: bool) -> String {
        match self {
            CodegenOption {
                side: Side::Client,
                language: Language::TypeScript,
                ..
            } => args
                .iter()
                .map(|req| {
                    if has_type {
                        let ty = chitin_util::to_typescript_type(&req.ty);
                        format!("{}: {}", req.name, ty)
                    } else {
                        req.name.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join(", "),
            CodegenOption {
                side: Side::Server { .. },
                language: Language::Rust,
                ..
            } => args
                .iter()
                .map(|req| {
                    if has_type {
                        format!("{}: {}", req.name, req.ty)
                    } else {
                        req.name.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join(", "),
            _ => {
                unimplemented!()
            }
        }
    }
}

impl Leaf {
    pub fn codegen<T: std::io::Write>(
        &self,
        option: &CodegenOption,
        path: &mut Vec<String>,
        stream: &mut T,
    ) -> std::io::Result<()> {
        path.push(self.name.clone());
        match option {
            CodegenOption {
                side: Side::Client,
                language: Language::TypeScript,
                ..
            } => {
                write!(
                    stream,
                    "    async {}({}): Promise<{}> {{\n",
                    self.name.to_camel_case(),
                    option.gen_args(&self.args, true),
                    option.gen_response_ty(&self.response_ty)
                )?;
                write!(
                    stream,
                    "        return JSON.parse(await this.fetchResult({}));\n",
                    chitin_util::gen_enum_json(path, &self.args)
                )?;
                write!(stream, "    }}\n")?;
            }
            CodegenOption {
                side: Side::Server { context },
                language: Language::Rust,
                ..
            } => {
                write!(
                    stream,
                    "    async fn {}(&self, context: {}, {}) -> {};\n",
                    self.name.to_snake_case(),
                    context,
                    option.gen_args(&self.args, true),
                    option.gen_response_ty(&self.response_ty)
                )?;
            }
            _ => {
                unimplemented!()
            }
        }
        path.pop();
        Ok(())
    }
}

impl ChitinEntry {
    pub fn root_codegen<T: std::io::Write>(
        &self,
        option: &CodegenOption,
        stream: &mut T,
    ) -> std::io::Result<()> {
        self.codegen(option, &mut vec![], stream)
    }
    pub fn codegen<T: std::io::Write>(
        &self,
        option: &CodegenOption,
        path: &mut Vec<String>,
        stream: &mut T,
    ) -> std::io::Result<()> {
        match option {
            CodegenOption {
                side: Side::Client,
                language: Language::TypeScript,
                ..
            } => {
                write!(stream, "export class {} {{\n", self.name)?;
                write!(stream, "    fetchResult: Fetcher;\n")?;
                for router in &self.routers {
                    write!(
                        stream,
                        "    {}: {}\n",
                        router.name.to_camel_case(),
                        router.name
                    )?;
                }
                write!(stream, "    constructor(fetcher: Fetcher){{\n")?;
                write!(stream, "        this.fetchResult = fetcher\n")?;
                for router in &self.routers {
                    write!(
                        stream,
                        "        this.{} = new {}(fetcher)\n",
                        router.name.to_camel_case(),
                        router.name
                    )?;
                }
                write!(stream, "    }}\n")?;

                for leaf in &self.leaves {
                    leaf.codegen(option, path, stream)?;
                }
                write!(stream, "}}\n\n")?;

                for router in &self.routers {
                    path.push(router.variant_name.as_ref().unwrap().clone());
                    router.codegen(option, path, stream)?;
                    path.pop();
                }
            }
            CodegenOption {
                side: Side::Server { context },
                language: Language::Rust,
                error,
            } => {
                for router in &self.routers {
                    path.push(router.variant_name.as_ref().unwrap().clone());
                    router.codegen(option, path, stream)?;
                    path.pop();
                }
                write!(stream, "#[async_trait]\npub trait {}Router {{\n", self.name)?;
                for router in &self.routers {
                    write!(
                        stream,
                        "    type {}Router: {}Router + Sync;\n",
                        router.name, router.name
                    )?;
                }
                for leaf in &self.leaves {
                    leaf.codegen(option, path, stream)?;
                }
                for router in &self.routers {
                    write!(
                        stream,
                        "   fn {}_router(&self) -> &Self::{}Router;\n",
                        router.variant_name.as_ref().unwrap().to_snake_case(),
                        router.name,
                    )?;
                }
                write!(
                    stream,
                    "    async fn handle(&self, context: {}, query: {}) -> Result<(String, Option<{}>), Error> {{\n",
                    context,
                    self.name,
                    error,
                )?;
                write!(stream, "        match query {{\n")?;
                for leaf in &self.leaves {
                    write!(
                        stream,
                        "             {}::{} {{ {} }} => {{\n",
                        self.name,
                        leaf.name,
                        option.gen_args(&leaf.args, false)
                    )?;
                    write!(
                        stream,
                        "                 let resp = self.{}(context, {}).await;\n",
                        &leaf.name.to_snake_case(),
                        option.gen_args(&leaf.args, false)
                    )?;
                    write!(
                        stream,
                        "                 let s = serde_json::to_string(&resp)?;\n"
                    )?;
                    write!(stream, "                 Ok((s, resp.err()))\n")?;
                    write!(stream, "            }}\n")?;
                }
                for router in &self.routers {
                    write!(
                        stream,
                        "             {}::{}(query) => {{\n",
                        self.name,
                        router.variant_name.as_ref().unwrap()
                    )?;
                    write!(
                        stream,
                        "                 self.{}_router().handle(context, query).await\n",
                        router.variant_name.as_ref().unwrap().to_snake_case()
                    )?;
                    write!(stream, "            }}\n")?;
                }
                write!(stream, "        }}\n")?;
                write!(stream, "    }}\n")?;
                write!(stream, "}}\n")?;
            }
            _ => {
                unimplemented!()
            }
        }
        Ok(())
    }
}
