use chitin::ChitinCodegen;

mod query;
use query::{RootQuery, UserQuery};

// mod hand_written;

fn main() {
    println!("{}", RootQuery::get_router_name());
    println!("{:?}", RootQuery::get_entries());

    println!("{}", UserQuery::get_router_name());
    println!("{:?}", UserQuery::get_entries());
}
