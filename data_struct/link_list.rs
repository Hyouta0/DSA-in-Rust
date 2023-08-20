use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link,
}
impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(Node) => {
                self.head = Node.next;
                Some(Node.elem)
            }
        }
    }
}
#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}
#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn node_test() {
        let mut i = List::new();
        i.push(4);
        i.push(5);
        println!("list is : {:?}", i);
        let j = i.pop().unwrap();
        println!("{}, is poped form list {:?}", j, i);
    }
}
