use std::io;

fn take_input()->i32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn selection_sort(nums:&mut Vec<i32>){
    let size = nums.len();
    for i in 0..size-1{
        let mut smallest = i;
        for j in i+1..size{
            if nums[j] < nums[smallest]{
                smallest = j; 
            }
        }
        if smallest != i{
            nums.swap(i,smallest);
        }
    }
}

fn main(){
    println!("Enter the size of the array");
    let size=take_input();
    println!("Enter {size} elements in the array");
    let mut nums:Vec<i32> = Vec::new();
    for _ in 0..size{
        nums.push(take_input());
    }
    println!("Before Sorting:");
    for i in &nums{
        print!("{}, ",i);
    }
    selection_sort(&mut nums);
    println!("\nAfter Sorting:");
    for i in &nums{
        print!("{}, ",i);
    }
    print!("\n");

}