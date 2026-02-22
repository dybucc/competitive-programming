use std::io;

fn main() {
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let [wc, hc, ws, hs] = b
        .split_ascii_whitespace()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        return;
    };
    println!("{}", {
        if wc.saturating_sub(ws) >= 2 && hc.saturating_sub(hs) >= 2 {
            "1"
        } else {
            "0"
        }
    });
}
