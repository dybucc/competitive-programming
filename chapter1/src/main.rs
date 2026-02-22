use std::cmp::Ordering::*;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let [n, m] = buf
        .split_ascii_whitespace()
        .map(|e| e.parse::<usize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        return;
    };
    println!("{}", {
        match m.cmp(&n) {
            Less => format!(
                "Dr. Chaz needs {} more {} of chicken!",
                n - m,
                if n - m == 1 { "piece" } else { "pieces" },
            ),
            Greater => format!(
                "Dr. Chaz will have {} of chicken left over!",
                format_args!("{} {}", m - n, if m - n == 1 { "piece" } else { "pieces" })
            ),
            _ => "".to_string(),
        }
    });
}
