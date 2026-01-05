use std::io;

fn take_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn merge_sort(start: usize, end: usize, nums: &mut Vec<i32>) {
    if start < end {
        let mid = (start + end) / 2;

        merge_sort(start, mid, nums);
        merge_sort(mid + 1, end, nums);

        merge(start, mid, end, nums);
    }
}

fn merge(start: usize, mid: usize, end: usize, nums: &mut Vec<i32>) {
    let a_size = mid - start + 1;
    let b_size = end - mid;

    let mut a = Vec::with_capacity(a_size);
    let mut b = Vec::with_capacity(b_size);

    for i in 0..a_size {
        a.push(nums[start + i]);
    }

    for i in 0..b_size {
        b.push(nums[mid + 1 + i]);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    while i < a_size && j < b_size {
        if a[i] <= b[j] {
            nums[k] = a[i];
            i += 1;
        } else {
            nums[k] = b[j];
            j += 1;
        }
        k += 1;
    }

    while i < a_size {
        nums[k] = a[i];
        i += 1;
        k += 1;
    }

    while j < b_size {
        nums[k] = b[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    println!("Enter the size of the array:");
    let size = take_input() as usize;

    let mut nums = Vec::with_capacity(size);
    println!("Enter {size} elements:");
    for _ in 0..size {
        nums.push(take_input());
    }

    println!("Before Sorting:");
    for i in &nums {
        print!("{}, ", i);
    }

    if !nums.is_empty() {
        merge_sort(0, nums.len() - 1, &mut nums);
    }

    println!("\nAfter Sorting:");
    for i in &nums {
        print!("{}, ", i);
    }
}
