use chitin::{ChitinRouter, Entry, Request};

#[derive(ChitinRouter)]
enum RootQuery {
    #[chitin(request, response = "Vec<Article>")]
    AskArticles { board_id: i32, count: u32 }, //, count: usize },
    #[chitin(request, response = "Board")]
    AskBoard { board_id: i32 },
}

fn main() {
    println!("{}", RootQuery::get_router_name());
    println!("{:?}", RootQuery::get_entries());
}
