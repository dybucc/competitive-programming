use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let [a, b, c, d, e] = buf
        .split_ascii_whitespace()
        .map(|e| e.parse::<usize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        return;
    };
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let s = buf.trim().parse::<usize>().unwrap();
    println!("{}", {
        let (a, b, c, d, e) = (a.., b..a, c..b, d..c, e..d);
        if a.contains(&s) {
            "A"
        } else if b.contains(&s) {
            "B"
        } else if c.contains(&s) {
            "C"
        } else if d.contains(&s) {
            "D"
        } else if e.contains(&s) {
            "E"
        } else {
            "F"
        }
    });
}
