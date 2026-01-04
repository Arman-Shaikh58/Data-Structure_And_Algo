use std::io;

fn take_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn insertion_sort(nums: &mut Vec<i32>) {
    let size = nums.len();

    for i in 1..size {
        let key = nums[i];
        let mut j = i;

        while j > 0 && nums[j - 1] > key {
            nums[j] = nums[j - 1];
            j -= 1;
        }

        nums[j] = key;
    }
}

fn main() {
    println!("Enter the size of the array");
    let size = take_input() as usize;

    println!("Enter {size} elements in the array");
    let mut nums: Vec<i32> = Vec::new();

    for _ in 0..size {
        nums.push(take_input());
    }

    println!("Before Sorting:");
    for i in &nums {
        print!("{}, ", i);
    }

    insertion_sort(&mut nums);

    println!("\nAfter Sorting:");
    for i in &nums {
        print!("{}, ", i);
    }
    println!();
}
