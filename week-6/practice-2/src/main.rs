use std::io;

fn checker(){
   
    println!("\nEnter a Character:");
    let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read input");
let ch:char = input.trim().parse().expect("Invalid input");

if ch >= '0' && ch <= '9'

{
    println!("character '{}' is a digit ",ch );
}
else 
{
println!("character '{}' is not a digit ",ch);
}
}


fn main() {
    //calling function
    println!("Welcome! This program check whether a character variable contains a digit or");
    checker()
}
