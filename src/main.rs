use std::io;

fn round_number(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10.0_f64.powi(decimal_places as i32);
    (value * multiplier).round() / multiplier
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut number_one = String::new();
    let mut number_two = String::new();
    let mut user = String::new();

    println!("addition (+), subtraction (-), multiplication (*), division (/)");
    io::stdin().read_line(&mut user)?;
    let user: &str = &user.trim();

    println!("Enter number one:");
    io::stdin().read_line(&mut number_one)?;
    let one = number_one.trim().parse::<f64>()?;

    println!("Enter number two:");
    io::stdin().read_line(&mut number_two)?;
    let two = number_two.trim().parse::<f64>()?;

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
    
    Ok(())
}