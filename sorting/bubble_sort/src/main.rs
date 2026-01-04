use std::io;

fn take_input_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().unwrap()
}

fn bubble_sort(nums: &mut Vec<i32>) {
    let size = nums.len();
    for i in 0..size {
        for j in 0..size - i - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}
fn main() {
    println!("Enter the length of the array");
    let size = take_input_i32();
    let mut nums: Vec<i32> = Vec::new();
    println!("Enter {size} numbers seprated by space");
    for _i in 0..size {
        nums.push(take_input_i32());
    }
    println!("Before Sorting");
    for i in &nums {
        print!("{}, ", i);
    }
    bubble_sort(&mut nums);
    println!("\nAfter sorting");
    for i in &nums {
        print!("{}, ", i);
    }
    println!("");
}
