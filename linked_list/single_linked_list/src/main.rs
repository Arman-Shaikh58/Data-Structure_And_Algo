use std::io;

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

    fn push_front(&mut self, val: i32) {
        let new_node = Box::new(Node {
            value: val,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn print_list(&self) {
        let mut current_node = &self.head;

        while let Some(node) = current_node {
            print!("{} -> ", node.value);
            current_node = &node.next;
        }
        print!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push_front(10);
    list.push_front(20);
    list.push_front(30);

    list.print_list();
}

fn take_input_i32() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falied to take input");
    input.trim().parse().expect("Failed to parse input")
}
