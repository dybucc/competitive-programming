use std::io;

fn main() {
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let [a, b, c, n] = b
        .split_ascii_whitespace()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        return;
    };
    println!("{}", {
        if n >= 3 && a >= 1 && b >= 1 && c >= 1 && (a + b + c) >= n {
            "YES"
        } else {
            "NO"
        }
    });
}
