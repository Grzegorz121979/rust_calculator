// #![allow(unused)]

use std::io;

fn round_number(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10.0_f64.powi(decimal_places as i32);
    (value * multiplier).round() / multiplier
}

fn main() {
    let mut number_one = String::new();
    let mut number_two = String::new();
    let mut user = String::new();
    
    println!("addition (+), subtraction (-), multiplication (*), division (/)");
    io::stdin().read_line(&mut user).unwrap();
    let user: &str = &user.trim();
    
    println!("Enter number one:");
    io::stdin().read_line(&mut number_one).unwrap();
    let one: f64 = number_one.trim().parse().unwrap();
    
    println!("Enter number two:");
    io::stdin().read_line(&mut number_two).unwrap();
    let two: f64 = number_two.trim().parse().unwrap();

    match user { 
        "+" => {
            let result = one + two;
            let round_2 = round_number(result, 2);
            println!("The result is: {}", round_2);
        },
        "-" => {
            let result = one - two;
            let round_2 = round_number(result, 2);
            println!("The result is: {}", round_2);
        }
        "*" => {
            let result = one * two;
            let round_2 = round_number(result, 2);
            println!("The result is: {}", round_2);
        }
        "/" => {
            let result = one / two;
            let round_2 = round_number(result, 2);
            println!("The result is: {}", round_2);
        }
        _ => println!("Error"),
    }
}
