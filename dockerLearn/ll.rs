struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_front(&mut self, value: i32) {
        let old_head = self.head.take();
        let new_node = Box::new(Node {
            value: value,
            next: old_head,
        });
        self.head = Some(new_node);
    }

    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{}", node.value);
            current = &node.next;
        }
    }
}

fn main() {
    println!("Tutorial for linked list.");
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.print();
}
