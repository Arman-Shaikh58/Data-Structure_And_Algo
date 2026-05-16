use std::io;

enum UserInput {
    INSERT,
    REMOVE,
    PRINT,
    EXIT,
    INVALID,
}

fn take_input_i32() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to tkae input");
    input
        .trim()
        .parse()
        .expect("Failed to parse input enter a valid number")
}

struct Node {
    val: i32,
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
        let node = Box::new(Node {
            val: val,
            next: self.head.take(),
        });

        self.head = Some(node);
    }

    fn pop_front(&mut self) {
        if let Some(mut b_node) = self.head.take() {
            println!("Removed Element: {}", b_node.val);
            self.head = b_node.next.take();
        } else {
            println!("List is Empty");
        }
    }

    fn print_list(&self) {
        let mut current = &self.head;

        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = &node.next;
        }
    }
}

fn parse_command() -> UserInput {
    match take_input_i32() {
        1 => UserInput::INSERT,
        2 => UserInput::REMOVE,
        3 => UserInput::PRINT,
        4 => UserInput::EXIT,
        _ => UserInput::INVALID,
    }
}

fn main() {
    let mut list = LinkedList::new();
    loop {
        println!("\n");
        print!("1. Insert\n2. Remove\n3. Print\n4. Exit\n");
        print!("Enter Your Choice: \n");
        match parse_command() {
            UserInput::INSERT => {
                println!("Enter the number: ");
                let val = take_input_i32();
                list.push_front(val);
            }
            UserInput::REMOVE => {
                list.pop_front();
            }
            UserInput::PRINT => {
                list.print_list();
            }
            UserInput::EXIT => {
                break;
            }
            UserInput::INVALID => {
                println!("Unvalid Choice");
            }
        }
    }
}
