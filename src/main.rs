use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut num_one = String::new();
    println!("Enter number one:");
    io::stdin().read_line(&mut num_one)?;
    let num_one = num_one.trim().parse::<f32>()?;
    
    println!("{}", num_one + 1.0);
    
    Ok(())
}
