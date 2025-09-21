#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take().map(Box::new),
        };
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next.map(|node| *node);
            Some(node.value)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut node = self.head.as_ref();
        while let Some(n) = node {
            count += 1;
            node = n.next.as_ref().map(|n| &**n);
        }
        count
    }
}
