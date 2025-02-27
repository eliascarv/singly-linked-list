use std::fmt;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { data, next: None }
    }
}

struct List<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { head: None, len: 0 }
    }

    fn push(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));
        match self.head {
            None => self.head = Some(new_node),
            Some(ref mut head) => {
                let mut current = head;
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(new_node)
            }
        }
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(mut head) => {
                self.head = head.next.take();
                self.len -= 1;
                Some(head.data)
            }
        }
    }
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut i = 0;
        let mut current = &self.head;
        while let Some(ref node) = current {
            i += 1;
            if i == self.len {
                write!(f, "{}", node.data)?;
            } else {
                write!(f, "{}, ", node.data)?;
            }
            current = &node.next;
        }
        write!(f, "]")
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    println!("List: {}", &list);
    list.pop();
    println!("List: {}", &list);
}
