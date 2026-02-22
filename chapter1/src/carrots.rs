use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    io::stdout()
        .write_all(buf.split_ascii_whitespace().last().unwrap().as_bytes())
        .unwrap();
}
