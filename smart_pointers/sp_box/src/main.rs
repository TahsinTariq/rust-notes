// mod sp_box;

// use sp_box::List::{Cons, Nil};
use sp_box::List;

enum WebEvent {
    PageLoad,
    Click { x: i64, y: i64 },
}

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    let en = WebEvent::Click { x: 1, y: 2 };
}
