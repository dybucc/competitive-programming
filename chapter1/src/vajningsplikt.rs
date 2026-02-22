use std::io;

fn main() {
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    #[derive(Clone, Copy)]
    enum Dir {
        North,
        South,
        East,
        West,
    }
    use Dir::*;
    let [mine, next, other] = b
        .split_ascii_whitespace()
        .map(|e| {
            match e {
                "North" => Some(North),
                "South" => Some(South),
                "East" => Some(East),
                "West" => Some(West),
                _ => None,
            }
            .unwrap()
        })
        .collect::<Vec<_>>()[..]
    else {
        return;
    };
    println!("{}", {
        match (mine, next, other) {
            (North, South, West) | (North, East, South) | (North, East, West) => "Yes",
            (South, North, East) | (South, West, East) | (South, West, North) => "Yes",
            (East, West, North) | (East, South, North) | (East, South, West) => "Yes",
            (West, East, South) | (West, North, South) | (West, North, East) => "Yes",
            _ => "No",
        }
    })
}
