use std::{collections::HashMap, hash::RandomState, io};

fn main() {
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let [m, d] = b.split_ascii_whitespace().collect::<Vec<_>>()[..] else {
        return;
    };
    let map: HashMap<&str, usize, RandomState> = HashMap::from_iter([
        ("JAN", 0_usize),
        ("FEB", 0_usize),
        ("MAR", 0_usize),
        ("APR", 0_usize),
        ("MAY", 0_usize),
        ("JUN", 0_usize),
        ("JUL", 0_usize),
        ("AUG", 0_usize),
        ("SEP", 0_usize),
        ("OCT", 31_usize),
        ("NOV", 0_usize),
        ("DEC", 25_usize),
    ]);
    println!("{}", {
        if m == "OCT" || m == "DEC" {
            if *map.get(m).unwrap() == d.parse().unwrap()
                || *map.get(m).unwrap() == d.parse().unwrap()
            {
                "yup"
            } else {
                "nope"
            }
        } else {
            "nope"
        }
    });
}
