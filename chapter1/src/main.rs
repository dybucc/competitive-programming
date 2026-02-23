use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let [x, y] = buf
        .split_ascii_whitespace()
        .map(|e| e.parse::<isize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        return;
    };
}
