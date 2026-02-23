use std::io::{self, BufReader, Read};

fn main() {
    let mut buf = String::new();
    let mut reader = BufReader::new(io::stdin());
    reader.read_to_string(&mut buf).unwrap();
    let [a, b, ab] = buf
        .split_terminator('\n')
        .skip(1)
        .map(|e| e.parse::<isize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        return;
    };
    println!(
        "{}",
        if (a - b).abs() != ab && a - b == ab {
            "JEDI"
        } else if (a - b).abs() == ab && a - b != ab {
            "SITH"
        } else {
            "VEIT EKKI"
        }
    );
}
