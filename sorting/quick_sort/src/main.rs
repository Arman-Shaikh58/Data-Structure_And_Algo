use std::io;

fn take_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn quick_sort(nums: &mut Vec<i32>, low: isize, high: isize) {
    if low < high {
        let pivot_idx = partition(nums, low, high);
        quick_sort(nums, low, pivot_idx - 1);
        quick_sort(nums, pivot_idx + 1, high);
    }
}

fn partition(nums: &mut Vec<i32>, low: isize, high: isize) -> isize {
    let pivot = nums[high as usize];
    let mut i = low;

    for j in low..high {
        if nums[j as usize] <= pivot {
            nums.swap(i as usize, j as usize);
            i += 1;
        }
    }

    nums.swap(i as usize, high as usize);
    i
}

fn main() {
    println!("Enter the size of array");
    let size = take_input() as usize;

    println!("Enter {} elements in the array", size);
    let mut nums: Vec<i32> = Vec::new();

    for _ in 0..size {
        nums.push(take_input());
    }

    println!("Before Sorting:");
    for i in &nums {
        print!("{i}, ");
    }

    if size > 0 {
        quick_sort(&mut nums, 0, (size - 1) as isize);
    }

    println!("\nAfter Sorting:");
    for i in &nums {
        print!("{i}, ");
    }
}
