#[derive(Debug)]
pub struct Request {
    pub req_type: String,
    pub name: String,
}

#[derive(Debug)]
pub enum Entry {
    Leaf {
        name: String,
        response_type: String,
        request: Vec<Request>
    },
    Node {
        name: String,
        enum_name: String
    }
}

pub trait ChitinRouter {
    fn get_entries() -> Vec<Entry>;
}

