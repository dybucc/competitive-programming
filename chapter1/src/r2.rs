use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let [r1, s] = buf
        .split_ascii_whitespace()
        .map(|elem| elem.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()[..]
    else {
        return;
    };
    println!("{}", 2 * s - r1);
}
