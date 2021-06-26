pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
