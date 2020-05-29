use chitin::{ChitinRouter, Entry};

#[derive(ChitinRouter)]
enum RootQuery {
    #[chitin(request, response = "String")]
    AskBoard { board_id: i32 },
}

fn main() {
    println!("{:?}", RootQuery::get_entries());
}
