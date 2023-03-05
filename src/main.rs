use std::collections::HashSet;
use std::io;

// Function to compute the sum of squares of digits in a number
fn sum_of_squares(n: i32) -> i32 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }
    sum
}

// Function to check if a number is a happy number
fn is_happy_number(n: i32) -> bool {
    let mut set = HashSet::new();
    let mut n = n;
    while n != 1 {
        if set.contains(&n) {
            return false;
        }
        set.insert(n);
        n = sum_of_squares(n);
    }
    true
}

fn main() {
    println!("Enter a number to check if it's a happy number:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let num = input.trim().parse::<i32>().unwrap();
    if is_happy_number(num) {
        println!("{} is a happy number!", num);
    } else {
        println!("{} is not a happy number.", num);
    }
}
