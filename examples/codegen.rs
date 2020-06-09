use chitin::{ChitinCodegen, CodegenOption};
mod query;
use query::RootQuery;
mod model;

fn main() {
    // println!("{}", RootQuery::get_router_name());
    // println!("{:?}", RootQuery::get_entries());

    // println!("{}", UserQuery::get_router_name());
    // println!("{:?}", UserQuery::get_entries());

    println!("use async_trait::async_trait;");
    println!("use crate::query::*;");
    println!("use serde_json::error::Error;");
    println!("{}", RootQuery::codegen(&CodegenOption::Client));
}
