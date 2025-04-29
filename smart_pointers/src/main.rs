// struct Node {
//     value: i32,
//     next: Option<Box<Node>>,
// }

use std::rc::Rc;

enum List {
    Node(i32, Rc<List>),
    Nil
}

fn main() {
    // println!("Hello, world!");
    let a = Rc::new(List::Node(1, Rc::new(List::Node(2, Rc::new(List::Nil)))));
    let b = List::Node(9, Rc::clone(&a));
    let c = List::Node(8, Rc::clone(&a));
}
