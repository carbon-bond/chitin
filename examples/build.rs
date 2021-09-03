#[path = "src/model.rs"]
mod model;
#[path = "src/query.rs"]
mod query;
use chitin::CodegenOption;
use chitin::{Language, Side};
use query::RootQuery;
use std::fs::File;
use std::io::prelude::*;

#[cfg(not(debug_assertions))]
fn main() -> std::io::Result<()> {
    Ok(())
}
#[cfg(debug_assertions)]
fn main() -> std::io::Result<()> {
    let chitin_entry = RootQuery::get_root_entry();

    let server_option = CodegenOption {
        side: Side::Server {
            context: "crate::Ctx",
        },
        language: Language::Rust,
        error: "String",
    };
    let mut server_file = File::create("src/api_trait.rs")?;
    server_file.write_all(b"use async_trait::async_trait;\n")?;
    server_file.write_all(b"use crate::query::*;\n")?;
    server_file.write_all(b"use serde_json::error::Error;\n")?;
    chitin_entry.root_codegen(&server_option, &mut server_file)?;

    let client_option = CodegenOption {
        side: Side::Client,
        language: Language::TypeScript,
        error: "String",
    };
    let mut client_file = File::create("client/api_trait.ts")?;
    client_file.write_all(client_option.prelude().as_bytes())?;
    client_file.write_all(model::gen_typescript().as_bytes())?;
    chitin_entry.root_codegen(&client_option, &mut client_file)?;
    Ok(())
}
