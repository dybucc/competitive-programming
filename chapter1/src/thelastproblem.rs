use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    println!("Thank you, {}, and farewell!", buffer.trim());
}
