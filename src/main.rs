use std::{borrow::Borrow, rc::Rc};

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
    println!("List's head -> {:?}", list.head);
}
