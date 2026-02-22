use std::io;

fn main() {
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let [n, k] = b
        .split_ascii_whitespace()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        return;
    };
    b.clear();
    io::stdin().read_line(&mut b).unwrap();
    let [d, s] = b
        .split_ascii_whitespace()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        return;
    };
    println!("{}", {
        let output = (n * d - s * k) as f64 / (n - k) as f64;
        if !(0. ..=100.).contains(&output) {
            "impossible".to_string()
        } else {
            output.to_string()
        }
    });
}
