use std::io;

const SIZE: u32 = 10;

struct Queue {
    queue: Vec<i32>,
}

impl Queue {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.queue.len() == 0
    }

    fn is_full(&self) -> bool {
        self.queue.len() == SIZE as usize
    }

    fn enqueue(&mut self, element: i32) -> Result<(), String> {
        if self.is_full() {
            return Err(String::from("The Queue is already full"));
        }
        self.queue.push(element);
        Ok(())
    }
    fn dequeue(&mut self) -> Result<i32, String> {
        if self.is_empty() {
            return Err(String::from("The Queue is Empty"));
        }
        let element = self.queue.remove(0);
        Ok(element)
    }
    fn display(&self) {
        if self.is_empty() {
            println!("Queue is Empty");
            return;
        }
        for i in &self.queue {
            print!("{}, ", i);
        }
    }
}
fn take_input_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let mut queue = Queue::new();
    loop {
        println!("Select your option");
        println!("1. Enqueue\n2. Dequeue\n3. Display\n4. Exit");
        match take_input_i32() {
            1 => {
                if queue.is_full() {
                    println!("The Queue is full");
                } else {
                    println!("Enter the element you want to insert");
                    let element = take_input_i32();
                    match queue.enqueue(element) {
                        Ok(()) => println!("Element Inserted SuccessFully"),
                        Err(err) => println!("Error: {}", err),
                    };
                }
            }
            2 => {
                match queue.dequeue() {
                    Ok(element) => println!("The Dequeued Element is: {}", element),
                    Err(err) => println!("Error: {}", err),
                };
            }
            3 => {
                queue.display();
            }
            4 => {
                break;
            }
            _ => {
                println!("Wrong Input");
            }
        };
    }
}
