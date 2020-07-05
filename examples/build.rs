#[path = "src/model.rs"]
mod model;
#[path = "src/query.rs"]
mod query;
use chitin::{ChitinCodegen, CodegenOption};
use query::RootQuery;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut server_file = File::create("src/api_trait.rs")?;
    server_file.write_all(b"use async_trait::async_trait;\n")?;
    server_file.write_all(b"use crate::query::*;\n")?;
    server_file.write_all(b"use serde_json::error::Error;\n")?;
    server_file.write_all(
        RootQuery::codegen(&CodegenOption::Server {
            error: "String",
            context: "crate::Ctx",
        })
        .as_bytes(),
    )?;

    let mut client_file = File::create("client/api_trait.ts")?;
    client_file.write_all(b"export type Option<T> = T | undefined | null;\n")?;
    client_file.write_all(
        b"export type Result<T, E> = {
    'Ok': T
} | {
    'Err': E
};\n",
    )?;
    client_file.write_all(model::gen_typescript().as_bytes())?;
    client_file
        .write_all(RootQuery::codegen(&CodegenOption::Client { error: "String" }).as_bytes())?;
    Ok(())
}
