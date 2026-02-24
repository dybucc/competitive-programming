use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let [x, y] = buf
        .split_ascii_whitespace()
        .map(|e| e.parse::<isize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        panic!();
    };
    let point_in_b = 1. / y as f64;
    println!(
        "{}",
        if x.is_positive() {
            let mut state_a = 0.;
            let mut state_b = 0_isize;
            while ((state_b - 1) as f64) < state_a - point_in_b {
                state_a -= point_in_b;
                state_b -= 1;
            }
            format!("{}", state_a.midpoint(state_a - point_in_b))
        } else if x.is_negative() {
            let mut state_a = 0.;
            let mut state_b = 0_isize;
            while ((state_b + 1) as f64) > state_a + point_in_b {
                state_a += point_in_b;
                state_b += 1;
            }
            format!("{}", state_a.midpoint(state_a + point_in_b))
        } else {
            "ALL GOOD".to_string()
        }
    )
}
