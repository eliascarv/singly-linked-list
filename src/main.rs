use std::fmt;

// Types

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

#[derive(Debug, Clone)]
struct List<T> {
    head: Link<T>,
    len: usize,
}

// Methods

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

impl<T> List<T> {
    fn new() -> Self {
        Self { head: None, len: 0 }
    }

    fn push_back(&mut self, data: T) {
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

    fn push_front(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.data
        })
    }
}

// Trait implementations

impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut i = 0;
        let mut current = &self.head;
        while let Some(node) = current {
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
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_front(0);
    println!("List: {}", &list);
    for (i, item) in list.enumerate() {
        println!("Item {}: {}", i, item)
    }
}
