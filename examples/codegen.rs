use chitin::{ChitinCodegen, CodegenOption};
mod query;
use query::RootQuery;
mod model;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 {
        if args[1] == "server" {
            println!("use async_trait::async_trait;");
            println!("use crate::query::*;");
            println!("use serde_json::error::Error;");
            println!("{}", RootQuery::codegen(&CodegenOption::Server));
        } else if args[1] == "client" {
            println!("{}", model::gen_typescript());
            println!("{}", RootQuery::codegen(&CodegenOption::Client));
        } else {
            panic!("未知的指令：{}", args[1]);
        }
    } else {
        panic!("用法： codegen server|client");
    }
}
