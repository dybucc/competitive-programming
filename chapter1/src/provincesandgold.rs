use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let b_pow: usize = buf
        .split_ascii_whitespace()
        .enumerate()
        .map(|(num, e)| {
            let e = e.parse::<usize>().unwrap();
            match num {
                0 => e * 3,
                1 => e * 2,
                2 => e,
                _ => panic!(),
            }
        })
        .sum();
    println!("{}", {
        match b_pow {
            8.. => match b_pow {
                6.. => "Province or Gold",
                3.. => "Province or Silver",
                _ => "Province or Copper",
            },

            5.. => match b_pow {
                6.. => "Duchy or Gold",
                3.. => "Duchy or Silver",
                _ => "Duchy or Copper",
            },

            2.. => match b_pow {
                6.. => "Estate or Gold",
                3.. => "Estate or Silver",
                _ => "Estate or Copper",
            },

            _ => "Copper",
        }
    });
}
