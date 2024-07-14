use std::{
    borrow::{Borrow, BorrowMut},
    rc::Rc,
};

#[derive(Debug)]
struct Node {
    val: u32,
    next: Rc<Option<Node>>,
}

struct LinkedList {
    head: Rc<Option<Node>>,
    tail: Rc<Option<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: Rc::new(Option::None),
            tail: Rc::new(Option::None),
        }
    }

    fn get(&self, index: i32) {}

    fn add_at_head(&mut self, val: i32) {
        match *self.head {
            None => {
                self.head = Rc::new(Option::Some(Node {
                    val: (val) as u32,
                    next: Rc::new(None),
                }))
            }
            Some(_) => {
                self.head = Rc::new(Option::Some(Node {
                    val: (val) as u32,
                    next: self.head.clone(),
                }))
            }
        }
    }
}

fn main() {
    let mut list: LinkedList = LinkedList::new();
    list.add_at_head(13);
    list.add_at_head(14);
    list.add_at_head(15);
    list.add_at_head(16);
    println!("List's head -> {:?}", list.head);
    list.get(12);
}
