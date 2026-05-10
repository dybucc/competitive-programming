use std::{
    env,
    fmt::Write as FmtWrite,
    io::{self, BufWriter, Write as IoWrite},
    process::Command,
};

use itertools::Itertools;

fn main() {
    let input = env::args_os().nth(2).unwrap().into_string().unwrap();
    let sort_order = env::args_os().nth(3).unwrap().into_string().unwrap();

    let ascendingly = if sort_order == "-a" {
        true
    } else if sort_order == "-d" {
        false
    } else {
        panic!()
    };
    let input: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut stdout = BufWriter::new(io::stdout().lock());

    let mut sorted = input.clone();
    sorted.sort_unstable_by(|n1, n2| match n1.cmp(n2) {
        order if !ascendingly => order.reverse(),
        order => order,
    });

    let len = input.len();
    let perms: Vec<_> = input.into_iter().permutations(len).collect();

    for perm in &perms {
        write!(stdout, "{perm:?}").unwrap();

        let mut input =
            perm.iter()
                .enumerate()
                .fold(format!("{}", perm.len()), |mut out, (i, num)| {
                    if i == perm.len() - 1 {
                        writeln!(out, "{num}").unwrap();
                    } else {
                        write!(out, "{num} ").unwrap();
                    }

                    out
                });

        sorted.iter().enumerate().for_each(|(i, n)| {
            if i == sorted.len() - 1 {
                write!(input, "{n} ")
            } else {
                writeln!(input, "{n}")
            }
            .unwrap();
        });

        Command::new("cargo").args(["r", "--", &input]);
    }
}
