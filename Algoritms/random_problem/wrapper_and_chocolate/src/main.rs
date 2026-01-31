use std::io;

fn take_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn cal_chocolate(chocolates: u32, wrapper: u32) -> u32 {
    if chocolates < wrapper {
        return 0;
    }

    let extra = chocolates / wrapper;
    let remaining_wrappers = chocolates % wrapper;

    extra + cal_chocolate(extra + remaining_wrappers, wrapper)
}

fn main() {
    println!("Enter the Number of chocolate you buied");
    let chocolate = take_input().parse().unwrap();
    println!("Enter the Number of Wrappers");
    let wrapper = take_input().parse().unwrap();
    let extra_chocolate = cal_chocolate(chocolate, wrapper);
    println!(
        "The extra chocolate you will have is {} and total chocolate will be {}",
        extra_chocolate,
        extra_chocolate + chocolate
    );
}
