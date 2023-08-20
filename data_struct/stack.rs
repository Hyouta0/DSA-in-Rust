pub struct Linklist<T> {
    head: Link<T>,
}
impl<T> Linklist<T> {
    fn new() -> Self {
        Self { head: None }
    }
    fn push(&mut self, element: T) {
        /*match &mut self.head {
            None => {
                self.head = Some(Box::new(Node {
                    element,
                    next: None,
                }))
            }
            Some(n) => {
                let current_head = Some(Box::new(Node {
                    element,
                    next: Some(n),
                }));
                self.head = current_head
            }
        }*/
        //let old_head = std::mem::replace(&mut self.head, None);
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.element
        })
        /*let old_head = self.head.take();
        match old_head {
            Some(n) => {
                self.head = n.next;
                Some(n.element)
            }
            None => None,
        }*/
    }

    fn peek(&self) -> Option<&T> {
        //self.head.as_ref().map(|n| &n.element);
        match &self.head {
            None => None,
            Some(n) => Some(&n.element),
        }
    }
}

struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_node() {
        let mut list: Linklist<i32> = Linklist::new();
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), Some(&2));
    }
}
