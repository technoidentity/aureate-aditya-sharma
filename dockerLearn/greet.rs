use std::io;
fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Error here!@");
    println!("Random print1!");
    println!("Hi, {name}! Greetings!");
}
